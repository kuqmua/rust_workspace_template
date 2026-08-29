#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "the flat source facade keeps its owner adjacent to implementation while declaring sibling modules"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Default)]
pub struct LeaseRegistry {
    inner: crate::tokio_lease_registry_rw_lock_arc::TokioLeaseRegistryRwLockArc,
}
impl LeaseRegistry {
    pub async fn heartbeat(
        &self,
        id: &crate::lease_id::LeaseId,
    ) -> crate::lease_heartbeat::LeaseHeartbeat {
        {
            let mut inner = self.inner.0.write().await;
            match inner.by_id.get_mut(id) {
                Some(entry) if entry.state != crate::lease_state::LeaseState::Stale => {
                    entry.heartbeat = crate::tokio_lease_instant::TokioLeaseInstant::from(
                        tokio::time::Instant::now(),
                    );
                    entry.state = crate::lease_state::LeaseState::Ready;
                    crate::lease_heartbeat::LeaseHeartbeat::Accepted
                }
                Some(_) | None => crate::lease_heartbeat::LeaseHeartbeat::Missing,
            }
        }
    }

    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn release(
        &self,
        id: &crate::lease_id::LeaseId,
    ) -> crate::lease_heartbeat::LeaseHeartbeat {
        let mut inner = self.inner.0.write().await;
        let Some(entry) = inner.by_id.remove(id) else {
            return crate::lease_heartbeat::LeaseHeartbeat::Missing;
        };
        let _removed = inner.by_key.remove(&entry.key);
        crate::lease_heartbeat::LeaseHeartbeat::Accepted
    }

    pub async fn reserve(
        &self,
        id: crate::lease_id::LeaseId,
        key: crate::lease_key::LeaseKey,
        maximum: crate::lease_registry_maximum_non_zero_usize::LeaseRegistryMaximumNonZeroUsize,
    ) -> crate::lease_reservation::LeaseReservation {
        {
            let mut inner = self.inner.0.write().await;
            if let Some(existing_id) = inner.by_key.get(&key)
                && inner
                    .by_id
                    .get(existing_id)
                    .is_some_and(|entry| entry.state != crate::lease_state::LeaseState::Stale)
            {
                return crate::lease_reservation::LeaseReservation::Existing(existing_id.clone());
            }
            #[allow(
                clippy::needless_collect,
                reason = "expired lease keys must be collected before mutating the registry"
            )]
            // ids must be owned before mutating both registry indexes
            let stale = inner
                .by_id
                .iter()
                .filter(|(_id, entry)| entry.state == crate::lease_state::LeaseState::Stale)
                .map(|(stale_id, _entry)| stale_id.clone())
                .collect::<Vec<_>>();
            stale.into_iter().fold((), |(), stale_id| {
                if let Some(entry) = inner.by_id.remove(&stale_id) {
                    let _removed = inner.by_key.remove(&entry.key);
                }
            });
            if inner.by_id.len().get() >= maximum.0.get() {
                return crate::lease_reservation::LeaseReservation::LimitReached;
            }
            if let Some(previous) = inner.by_id.remove(&id) {
                let _removed = inner.by_key.remove(&previous.key);
            }
            if let Some(previous_id) = inner.by_key.remove(&key) {
                let _removed = inner.by_id.remove(&previous_id);
            }
            let id_insertion = inner.by_key.try_insert(key.clone(), id.clone());
            if id_insertion.is_err() {
                return crate::lease_reservation::LeaseReservation::LimitReached;
            }
            let entry_insertion = inner.by_id.try_insert(
                id,
                crate::lease_entry::LeaseEntry {
                    heartbeat: crate::tokio_lease_instant::TokioLeaseInstant::from(
                        tokio::time::Instant::now(),
                    ),
                    key: key.clone(),
                    state: crate::lease_state::LeaseState::Reserved,
                },
            );
            if entry_insertion.is_err() {
                let _removed_id = inner.by_key.remove(&key);
                drop(inner);
                return crate::lease_reservation::LeaseReservation::LimitReached;
            }
            drop(inner);
            crate::lease_reservation::LeaseReservation::Reserved
        }
    }

    pub async fn stale(
        &self,
        timeout: crate::lease_stale_timeout_duration::LeaseStaleTimeoutDuration,
    ) -> crate::lease_ids::LeaseIds {
        crate::lease_ids::LeaseIds::from({
            let mut inner = self.inner.0.write().await;
            let now = tokio::time::Instant::now();
            let mut stale_ids = bounded_types::bounded_vec::BoundedVec::default();
            inner
                .by_id
                .iter_mut()
                .filter_map(|(id, entry)| {
                    (now.duration_since(entry.heartbeat.0) > timeout.0).then(|| {
                        entry.state = crate::lease_state::LeaseState::Stale;
                        id.clone()
                    })
                })
                .for_each(|id| stale_ids.push_max_capacity(id));
            stale_ids
        })
    }
}
#[cfg(test)]
mod tests {
    fn id(value: &str) -> crate::lease_id::LeaseId {
        crate::lease_id::LeaseId::try_from(value.to_owned())
            .expect("f1f58adc id invariant must hold")
    }
    fn lease_key(value: &str) -> crate::lease_key::LeaseKey {
        crate::lease_key::LeaseKey::try_from(value.to_owned())
            .expect("699f4283 key invariant must hold")
    }
    fn maximum() -> crate::lease_registry_maximum_non_zero_usize::LeaseRegistryMaximumNonZeroUsize {
        crate::lease_registry_maximum_non_zero_usize::LeaseRegistryMaximumNonZeroUsize::from(
            std::num::NonZeroUsize::MIN,
        )
    }

    #[tokio::test]
    async fn reservation_is_unique_by_key_and_limit() {
        let registry = super::LeaseRegistry::new();
        let first_id = id(constants_str::test_fixtures::TEST_LEASE_ID_ONE);
        let first_key = lease_key(constants_str::test_fixtures::TEST_LEASE_KEY_ONE);
        assert_eq!(
            registry
                .reserve(first_id.clone(), first_key.clone(), maximum())
                .await,
            crate::lease_reservation::LeaseReservation::Reserved
        );
        assert_eq!(
            registry
                .reserve(
                    id(constants_str::test_fixtures::TEST_LEASE_ID_TWO),
                    first_key,
                    maximum()
                )
                .await,
            crate::lease_reservation::LeaseReservation::Existing(first_id)
        );
        assert_eq!(
            registry
                .reserve(
                    id(constants_str::test_fixtures::TEST_LEASE_ID_TWO),
                    lease_key(constants_str::test_fixtures::TEST_LEASE_KEY_TWO),
                    maximum(),
                )
                .await,
            crate::lease_reservation::LeaseReservation::LimitReached
        );
    }

    #[tokio::test(start_paused = true)]
    async fn heartbeat_and_stale_transition_are_observable() {
        let registry = super::LeaseRegistry::new();
        let lease_id = id(constants_str::test_fixtures::TEST_LEASE_ID_ONE);
        let _reservation = registry
            .reserve(
                lease_id.clone(),
                lease_key(constants_str::test_fixtures::TEST_LEASE_KEY_ONE),
                maximum(),
            )
            .await;
        assert_eq!(
            registry.heartbeat(&lease_id).await,
            crate::lease_heartbeat::LeaseHeartbeat::Accepted
        );
        tokio::time::advance(std::time::Duration::from_secs(2u64)).await;
        let stale = registry
            .stale(
                crate::lease_stale_timeout_duration::LeaseStaleTimeoutDuration::try_from(
                    std::time::Duration::from_secs(1u64),
                )
                .expect(
                    "8cb64054 heartbeat_and_stale_transition_are_observable invariant must hold",
                ),
            )
            .await;
        assert_eq!(stale.as_ref(), std::slice::from_ref(&lease_id));
        assert_eq!(
            registry.heartbeat(&lease_id).await,
            crate::lease_heartbeat::LeaseHeartbeat::Missing
        );
    }
}
