#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_to_err_string::ToErrString,
)]
#[serde(from = "usize")]
pub struct BodySizeLimitBytes(usize);

impl BodySizeLimitBytes {
    pub(crate) const fn value(self) -> usize {
        self.0
    }
}
