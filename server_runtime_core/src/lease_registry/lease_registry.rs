use super::{
    LeaseEntry, LeaseHeartbeat, LeaseId, LeaseIds, LeaseKey, LeaseRegistryMaximumNonZeroUsize,
    LeaseReservation, LeaseStaleTimeoutDuration, LeaseState, TokioLeaseInstant,
    TokioLeaseRegistryRwLockArc,
};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Default)]
pub struct LeaseRegistry {
    inner: TokioLeaseRegistryRwLockArc,
}
impl LeaseRegistry {
    pub async fn heartbeat(&self, id: &LeaseId) -> LeaseHeartbeat {
        {
            let mut inner = self.inner.0.write().await;
            match inner.by_id.get_mut(id) {
                Some(entry) if entry.state != LeaseState::Stale => {
                    entry.heartbeat = TokioLeaseInstant::from(tokio::time::Instant::now());
                    entry.state = LeaseState::Ready;
                    LeaseHeartbeat::Accepted
                }
                Some(_) | None => LeaseHeartbeat::Missing,
            }
        }
    }

    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn release(&self, id: &LeaseId) -> LeaseHeartbeat {
        let mut inner = self.inner.0.write().await;
        let Some(entry) = inner.by_id.remove(id) else {
            return LeaseHeartbeat::Missing;
        };
        let _removed = inner.by_key.remove(&entry.key);
        LeaseHeartbeat::Accepted
    }

    pub async fn reserve(
        &self,
        id: LeaseId,
        key: LeaseKey,
        maximum: LeaseRegistryMaximumNonZeroUsize,
    ) -> LeaseReservation {
        {
            let mut inner = self.inner.0.write().await;
            if let Some(existing_id) = inner.by_key.get(&key)
                && inner
                    .by_id
                    .get(existing_id)
                    .is_some_and(|entry| entry.state != LeaseState::Stale)
            {
                return LeaseReservation::Existing(existing_id.clone());
            }
            #[allow(
                clippy::needless_collect,
                reason = "expired lease keys must be collected before mutating the registry"
            )]
            // ids must be owned before mutating both registry indexes
            let stale = inner
                .by_id
                .iter()
                .filter(|(_id, entry)| entry.state == LeaseState::Stale)
                .map(|(stale_id, _entry)| stale_id.clone())
                .collect::<Vec<_>>();
            stale.into_iter().fold((), |(), stale_id| {
                if let Some(entry) = inner.by_id.remove(&stale_id) {
                    let _removed = inner.by_key.remove(&entry.key);
                }
            });
            if inner.by_id.len().get() >= maximum.0.get() {
                return LeaseReservation::LimitReached;
            }
            if let Some(previous) = inner.by_id.remove(&id) {
                let _removed = inner.by_key.remove(&previous.key);
            }
            if let Some(previous_id) = inner.by_key.remove(&key) {
                let _removed = inner.by_id.remove(&previous_id);
            }
            let id_insertion = inner.by_key.try_insert(key.clone(), id.clone());
            if id_insertion.is_err() {
                return LeaseReservation::LimitReached;
            }
            let entry_insertion = inner.by_id.try_insert(
                id,
                LeaseEntry {
                    heartbeat: TokioLeaseInstant::from(tokio::time::Instant::now()),
                    key: key.clone(),
                    state: LeaseState::Reserved,
                },
            );
            if entry_insertion.is_err() {
                let _removed_id = inner.by_key.remove(&key);
                drop(inner);
                return LeaseReservation::LimitReached;
            }
            drop(inner);
            LeaseReservation::Reserved
        }
    }

    pub async fn stale(&self, timeout: LeaseStaleTimeoutDuration) -> LeaseIds {
        LeaseIds::from({
            let mut inner = self.inner.0.write().await;
            let now = tokio::time::Instant::now();
            let mut stale_ids = bounded_types::domain_types::vector::BoundedVec::default();
            inner
                .by_id
                .iter_mut()
                .filter_map(|(id, entry)| {
                    (now.duration_since(entry.heartbeat.0) > timeout.0).then(|| {
                        entry.state = LeaseState::Stale;
                        id.clone()
                    })
                })
                .for_each(|id| stale_ids.push_max_capacity(id));
            stale_ids
        })
    }
}
