#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    proc_macro_newtype::DerefInner,
    proc_macro_newtype::FromInner,
)]
pub(super) struct ArcSingleFlightRwLock(
    std::sync::Arc<std::sync::RwLock<crate::single_flight_inner::SingleFlightInner>>,
);
