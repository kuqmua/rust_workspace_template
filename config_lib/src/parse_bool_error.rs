#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::DebugTransparent,
    newtype::Display,
    newtype::FromInner,
)]
pub struct ParseBoolError(std::str::ParseBoolError);
