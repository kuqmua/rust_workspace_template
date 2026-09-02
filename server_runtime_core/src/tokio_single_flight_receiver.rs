#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    proc_macro_newtype::DerefInner,
    proc_macro_newtype::DerefMutInner,
    proc_macro_newtype::FromInner,
)]
pub(super) struct TokioSingleFlightReceiver(
    tokio::sync::watch::Receiver<crate::single_flight_signal::SingleFlightSignal>,
);
