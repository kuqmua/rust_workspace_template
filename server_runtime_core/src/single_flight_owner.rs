#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
#[must_use]
pub struct SingleFlightOwner {
    pub(super) inner: crate::arc_single_flight_rw_lock::ArcSingleFlightRwLock,
    pub(super) key: Option<crate::single_flight_key::SingleFlightKey>,
}
impl Drop for SingleFlightOwner {
    fn drop(&mut self) {
        let Some(key) = self.key.take() else {
            return;
        };
        let optional_sender = crate::write_inner::write_inner(&self.inner)
            .flights
            .remove(&key);
        if let Some(sender) = optional_sender {
            let _send_result = sender
                .0
                .send(crate::single_flight_signal::SingleFlightSignal::Retry);
        }
    }
}
