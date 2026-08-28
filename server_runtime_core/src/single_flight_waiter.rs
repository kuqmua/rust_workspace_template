use super::{SingleFlightWaitOutcome, TokioSingleFlightReceiver};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub struct SingleFlightWaiter(TokioSingleFlightReceiver);

impl SingleFlightWaiter {
    pub async fn wait(mut self) -> SingleFlightWaitOutcome {
        match self.0.0.changed().await {
            Ok(()) | Err(_) => SingleFlightWaitOutcome::Retry,
        }
    }
}
