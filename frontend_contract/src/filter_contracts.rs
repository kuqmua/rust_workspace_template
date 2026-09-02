#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    proc_macro_newtype::AsRefInner,
    proc_macro_newtype::FromInner,
)]
pub struct FilterContracts(&'static [crate::filter_operation::FilterOperation]);
