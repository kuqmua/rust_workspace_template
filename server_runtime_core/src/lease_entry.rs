#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, proc_macro_new::New)]
#[constructor(pub(crate))]
pub(super) struct LeaseEntry {
    heartbeat: crate::tokio_lease_instant::TokioLeaseInstant,
    key: crate::lease_key::LeaseKey,
    state: crate::lease_state::LeaseState,
}

impl LeaseEntry {
    pub(crate) fn into_key(self) -> crate::lease_key::LeaseKey {
        self.key
    }

    pub(crate) fn is_stale(&self) -> bool {
        self.state == crate::lease_state::LeaseState::Stale
    }

    pub(crate) fn mark_stale_if_expired(
        &mut self,
        instant: tokio::time::Instant,
        lease_stale_timeout_duration: crate::lease_stale_timeout_duration::LeaseStaleTimeoutDuration,
    ) -> bool {
        if instant.duration_since(*self.heartbeat) <= *lease_stale_timeout_duration {
            return false;
        }
        self.state = crate::lease_state::LeaseState::Stale;
        true
    }

    pub(crate) fn refresh(&mut self, instant: tokio::time::Instant) {
        self.heartbeat = crate::tokio_lease_instant::TokioLeaseInstant::from(instant);
        self.state = crate::lease_state::LeaseState::Ready;
    }
}
