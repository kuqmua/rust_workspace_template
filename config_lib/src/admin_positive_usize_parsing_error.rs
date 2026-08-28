#[derive(
    optimal_memory_layout::OptimalMemoryLayout, newtype::DebugTransparent, newtype::FromInner,
)]
pub struct AdminPositiveUsizeParsingError(crate::ParseIntError);
