#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Default)]
pub(super) struct LeaseRegistryInner {
    by_id: bounded_types::bounded_hash_map::BoundedHashMap<
        crate::lease_id::LeaseId,
        crate::lease_entry::LeaseEntry,
        { usize::MAX },
    >,
    by_key: bounded_types::bounded_hash_map::BoundedHashMap<
        crate::lease_key::LeaseKey,
        crate::lease_id::LeaseId,
        { usize::MAX },
    >,
}

#[allow(
    clippy::single_call_fn,
    reason = "the state owner centralizes paired-index invariants behind its private fields"
)]
impl LeaseRegistryInner {
    pub(super) fn heartbeat(
        &mut self,
        id: &crate::lease_id::LeaseId,
        now: tokio::time::Instant,
    ) -> crate::lease_heartbeat::LeaseHeartbeat {
        match self.by_id.get_mut(id) {
            Some(entry) if !entry.is_stale() => {
                entry.refresh(now);
                crate::lease_heartbeat::LeaseHeartbeat::Accepted
            }
            Some(_) | None => crate::lease_heartbeat::LeaseHeartbeat::Missing,
        }
    }

    pub(super) fn release(
        &mut self,
        id: &crate::lease_id::LeaseId,
    ) -> crate::lease_heartbeat::LeaseHeartbeat {
        let Some(entry) = self.by_id.remove(id) else {
            return crate::lease_heartbeat::LeaseHeartbeat::Missing;
        };
        let _removed = self.by_key.remove(&entry.into_key());
        crate::lease_heartbeat::LeaseHeartbeat::Accepted
    }

    pub(super) fn reserve(
        &mut self,
        id: crate::lease_id::LeaseId,
        key: &crate::lease_key::LeaseKey,
        maximum: crate::lease_registry_maximum_non_zero_usize::LeaseRegistryMaximumNonZeroUsize,
        now: tokio::time::Instant,
    ) -> crate::lease_reservation::LeaseReservation {
        if let Some(existing_id) = self.by_key.get(key)
            && self
                .by_id
                .get(existing_id)
                .is_some_and(|entry| !entry.is_stale())
        {
            return crate::lease_reservation::LeaseReservation::Existing(existing_id.clone());
        }
        #[allow(
            clippy::needless_collect,
            reason = "expired lease keys must be collected before mutating the registry"
        )]
        let stale = self
            .by_id
            .iter()
            .filter(|(_id, entry)| entry.is_stale())
            .map(|(stale_id, _entry)| stale_id.clone())
            .collect::<Vec<_>>();
        stale.into_iter().fold((), |(), stale_id| {
            if let Some(entry) = self.by_id.remove(&stale_id) {
                let _removed = self.by_key.remove(&entry.into_key());
            }
            // paired indexes have both been updated before the next stale id
        });
        if self.by_id.len().get() >= maximum.get() {
            return crate::lease_reservation::LeaseReservation::LimitReached;
        }
        if let Some(previous) = self.by_id.remove(&id) {
            let _removed = self.by_key.remove(&previous.into_key());
        }
        if let Some(previous_id) = self.by_key.remove(key) {
            let _removed = self.by_id.remove(&previous_id);
        }
        let id_insertion = self.by_key.try_insert(key.clone(), id.clone());
        if id_insertion.is_err() {
            return crate::lease_reservation::LeaseReservation::LimitReached;
        }
        let entry_insertion = self.by_id.try_insert(
            id,
            crate::lease_entry::LeaseEntry::new(
                crate::tokio_lease_instant::TokioLeaseInstant::from(now),
                key.clone(),
                crate::lease_state::LeaseState::Reserved,
            ),
        );
        if entry_insertion.is_err() {
            let _removed_id = self.by_key.remove(key);
            return crate::lease_reservation::LeaseReservation::LimitReached;
        }
        crate::lease_reservation::LeaseReservation::Reserved
    }

    pub(super) fn stale(
        &mut self,
        now: tokio::time::Instant,
        timeout: crate::lease_stale_timeout_duration::LeaseStaleTimeoutDuration,
    ) -> crate::lease_ids::LeaseIds {
        let mut stale_ids = bounded_types::bounded_vec::BoundedVec::default();
        self.by_id
            .iter_mut()
            .filter_map(|(id, entry)| {
                entry
                    .mark_stale_if_expired(now, timeout)
                    .then(|| id.clone())
            })
            .for_each(|id| stale_ids.push_max_capacity(id));
        crate::lease_ids::LeaseIds::from(stale_ids)
    }
}
