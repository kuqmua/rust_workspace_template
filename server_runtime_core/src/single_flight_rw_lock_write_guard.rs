#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype_deref_mut_target::DerefMutTarget,
    proc_macro_newtype_deref_target::DerefTarget,
    proc_macro_newtype_from_inner::FromInner,
)]
pub(super) struct SingleFlightRwLockWriteGuard<'value_lt>(
    std::sync::RwLockWriteGuard<'value_lt, crate::single_flight_inner::SingleFlightInner>,
);
