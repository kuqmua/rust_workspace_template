#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::Display,
    newtype::FromInner,
    newtype::GetInner,
)]
#[serde(from = "usize")]
pub struct BoundedVecLen(usize);
impl to_err_string::to_err_string::ToErrString for BoundedVecLen {
    fn to_err_string(&self) -> to_err_string::error_text::ErrorText {
        to_err_string::error_text::ErrorText::try_from(self.to_string())
            .unwrap_or_else(to_err_string::error_text::ErrorText::from)
    }
}
