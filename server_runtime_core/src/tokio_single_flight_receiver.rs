#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    proc_macro_newtype_deref_inner::DerefInner,
    proc_macro_newtype_deref_mut_inner::DerefMutInner,
    proc_macro_newtype_from_inner::FromInner,
)]
pub(super) struct TokioSingleFlightReceiver(
    tokio::sync::watch::Receiver<crate::single_flight_signal::SingleFlightSignal>,
);
