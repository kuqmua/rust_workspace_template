#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype::DerefMutTarget,
    proc_macro_newtype::DerefTarget,
    proc_macro_newtype::FromInner,
)]
pub(super) struct SingleFlightRwLockWriteGuard<'value_lt>(
    std::sync::RwLockWriteGuard<'value_lt, crate::single_flight_inner::SingleFlightInner>,
);
