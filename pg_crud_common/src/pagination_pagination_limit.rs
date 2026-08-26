#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::Display,
    newtype::FromInner,
    newtype::GetInner,
    newtype::ToErrString,
)]
#[serde(from = "i64")]
pub struct PaginationLimit(i64);
impl From<i32> for PaginationLimit {
    fn from(value: i32) -> Self {
        Self(value.into())
    }
}
