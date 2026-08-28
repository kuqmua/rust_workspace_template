#[derive(
    optimal_memory_layout::OptimalMemoryLayout, newtype::DebugTransparent, newtype::FromInner,
)]
pub struct AdminPositiveU64ParsingError(super::super::ParseIntError);
