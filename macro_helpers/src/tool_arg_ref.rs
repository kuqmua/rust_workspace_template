#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::DerefInner,
    newtype::FromInner,
)]
pub struct ToolArgRef<'lt>(&'lt str);
