use super::SingleFlightSignal;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, newtype::FromInner)]
pub(super) struct TokioSingleFlightSender(
    pub(super) tokio::sync::watch::Sender<SingleFlightSignal>,
);
