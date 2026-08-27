#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::Display,
    newtype::FromInner,
    newtype::GetInner,
)]
#[serde(from = "i32")]
pub struct UnsignedPartOfI32Raw(i32);

impl to_err_string::domain_types::ToErrString for UnsignedPartOfI32Raw {
    fn to_err_string(&self) -> to_err_string::domain_types::ErrorText {
        to_err_string::domain_types::ErrorText::try_from(self.to_string())
            .unwrap_or_else(to_err_string::domain_types::ErrorText::from)
    }
}
