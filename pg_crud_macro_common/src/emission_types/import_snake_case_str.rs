#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::AsRefStr,
    newtype::Display,
    newtype::FromInner,
)]
pub struct ImportSnakeCaseStr(&'static str);
