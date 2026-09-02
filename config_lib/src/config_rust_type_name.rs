#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_newtype::AsRefInner,
    proc_macro_newtype::FromInner,
)]
pub struct ConfigRustTypeName(&'static str);
