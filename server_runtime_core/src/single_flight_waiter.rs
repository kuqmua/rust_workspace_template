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

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_closed_single_flight_channel_returns_retry() {
        let (sender, receiver) =
            tokio::sync::watch::channel(crate::single_flight_signal::SingleFlightSignal::Running);
        let waiter = super::SingleFlightWaiter::from(
            crate::tokio_single_flight_receiver::TokioSingleFlightReceiver::from(receiver),
        );
        drop(sender);
        assert_eq!(
            waiter.wait().await,
            crate::single_flight_wait_outcome::SingleFlightWaitOutcome::Retry
        );
    }
}
