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
    proc_macro_newtype_display::Display,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_get_inner::GetInner,
    proc_macro_newtype_to_err_string::ToErrString,
)]
#[serde(from = "i64")]
pub struct PaginationLimit(i64);
impl From<i32> for PaginationLimit {
    fn from(value: i32) -> Self {
        Self(value.into())
    }
}
