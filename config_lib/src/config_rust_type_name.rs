#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::AsRefInner,
    newtype::FromInner,
)]
pub struct ConfigRustTypeName(&'static str);
