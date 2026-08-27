#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use super::{ArcSingleFlightRwLock, SingleFlightKey, SingleFlightSignal, write_inner};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
#[must_use]
pub struct SingleFlightOwner {
    pub(super) inner: ArcSingleFlightRwLock,
    pub(super) key: Option<SingleFlightKey>,
}
impl Drop for SingleFlightOwner {
    fn drop(&mut self) {
        let Some(key) = self.key.take() else {
            return;
        };
        let optional_sender = write_inner(&self.inner).flights.remove(&key);
        if let Some(sender) = optional_sender {
            let _send_result = sender.0.send(SingleFlightSignal::Retry);
        }
    }
}
