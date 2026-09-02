#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype::Display,
    proc_macro_newtype::FromInner,
)]
pub struct HttpRuntimeTestStatus(u16);
