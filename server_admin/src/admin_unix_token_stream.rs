#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    newtype::FromInner,
)]
#[serde(from = "u64")]
#[derive(generate_accessor::Getters)]
pub struct AdminUnixTokenStream(u64);
