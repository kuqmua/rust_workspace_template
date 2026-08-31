#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, generate_constructor::New)]
#[constructor(pub(crate))]
#[must_use]
pub struct SingleFlightOwner {
    inner: crate::arc_single_flight_rw_lock::ArcSingleFlightRwLock,
    key: Option<crate::single_flight_key::SingleFlightKey>,
}
impl Drop for SingleFlightOwner {
    fn drop(&mut self) {
        let Some(key) = self.key.take() else {
            return;
        };
        let optional_sender = crate::write_inner::write_inner(&self.inner).remove(&key);
        if let Some(sender) = optional_sender {
            let _send_result = sender.send(crate::single_flight_signal::SingleFlightSignal::Retry);
        }
    }
}
