#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::FromInner,
    newtype::ToErrString,
)]
#[serde(from = "u32")]
pub struct LocationTestCount(u32);
