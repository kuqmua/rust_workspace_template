#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    newtype::DerefMutTarget,
    newtype::DerefTarget,
    newtype::FromInner,
)]
pub(super) struct SingleFlightRwLockWriteGuard<'value_lt>(
    std::sync::RwLockWriteGuard<'value_lt, crate::single_flight_inner::SingleFlightInner>,
);
