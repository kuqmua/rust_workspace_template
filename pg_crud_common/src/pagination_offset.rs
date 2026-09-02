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
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype::Display,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::GetInner,
    proc_macro_newtype::ToErrString,
)]
#[serde(from = "i64")]
pub struct PaginationOffset(i64);
impl From<i32> for PaginationOffset {
    fn from(i32: i32) -> Self {
        Self(i32.into())
    }
}
