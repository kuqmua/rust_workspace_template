// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(
    Debug, serde::Serialize, serde::Deserialize, optimal_memory_layout::OptimalMemoryLayout,
)]
pub struct SerdeStruct {
    pub one: crate::location_test_text::LocationTestText,
    pub three: crate::location_test_count::LocationTestCount,
    pub two: crate::location_test_flag::LocationTestFlag,
}

impl to_err_string::to_err_string::ToErrString for SerdeStruct {
    fn to_err_string(&self) -> to_err_string::error_text::ErrorText {
        to_err_string::error_text::ErrorText::try_from(format!("{self:?}"))
            .unwrap_or_else(to_err_string::error_text::ErrorText::from)
    }
}
