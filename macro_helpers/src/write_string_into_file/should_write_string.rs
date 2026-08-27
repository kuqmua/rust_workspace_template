#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    newtype::FromInner,
    newtype::IntoInnerFrom,
    newtype::NotInner,
)]
pub struct ShouldWriteString(bool);
