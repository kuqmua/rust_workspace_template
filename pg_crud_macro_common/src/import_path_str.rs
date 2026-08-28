#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::Display,
    newtype::FromInner,
)]
pub struct ImportPathStr(&'static str);
