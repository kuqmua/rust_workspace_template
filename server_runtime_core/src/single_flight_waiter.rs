#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype_from_inner::FromInner,
)]
pub struct SingleFlightWaiter(crate::tokio_single_flight_receiver::TokioSingleFlightReceiver);

impl SingleFlightWaiter {
    pub async fn wait(mut self) -> crate::single_flight_wait_outcome::SingleFlightWaitOutcome {
        match self.0.changed().await {
            Ok(()) | Err(_) => crate::single_flight_wait_outcome::SingleFlightWaitOutcome::Retry,
        }
    }
}
