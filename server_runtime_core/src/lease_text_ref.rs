#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_newtype::DerefInner,
    proc_macro_newtype::FromInner,
)]
pub(super) struct LeaseTextRef<'value_lt>(&'value_lt str);
