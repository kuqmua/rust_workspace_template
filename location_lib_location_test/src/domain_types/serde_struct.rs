#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(
    Debug, serde::Serialize, serde::Deserialize, optimal_memory_layout::OptimalMemoryLayout,
)]
pub struct SerdeStruct {
    pub one: super::LocationTestText,
    pub three: super::LocationTestCount,
    pub two: super::LocationTestFlag,
}

impl to_err_string::domain_types::ToErrString for SerdeStruct {
    fn to_err_string(&self) -> to_err_string::domain_types::ErrorText {
        to_err_string::domain_types::ErrorText::try_from(format!("{self:?}"))
            .unwrap_or_else(to_err_string::domain_types::ErrorText::from)
    }
}
