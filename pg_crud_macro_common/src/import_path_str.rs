#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype::Display,
    proc_macro_newtype::FromInner,
)]
pub struct ImportPathStr(&'static str);
