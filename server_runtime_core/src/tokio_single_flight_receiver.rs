#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::DerefInner,
    newtype::DerefMutInner,
    newtype::FromInner,
)]
pub(super) struct TokioSingleFlightReceiver(
    tokio::sync::watch::Receiver<crate::single_flight_signal::SingleFlightSignal>,
);
