#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    newtype::DerefInner,
    newtype::FromInner,
)]
pub(super) struct ArcSingleFlightRwLock(
    std::sync::Arc<std::sync::RwLock<crate::single_flight_inner::SingleFlightInner>>,
);
