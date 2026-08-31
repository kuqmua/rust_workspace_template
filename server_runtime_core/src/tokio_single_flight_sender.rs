#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::DerefInner,
    newtype::FromInner,
)]
pub(super) struct TokioSingleFlightSender(
    tokio::sync::watch::Sender<crate::single_flight_signal::SingleFlightSignal>,
);
