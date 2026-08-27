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
    newtype::GetInner,
)]
#[serde(from = "bool")]
pub struct StdAdminBool(bool);
