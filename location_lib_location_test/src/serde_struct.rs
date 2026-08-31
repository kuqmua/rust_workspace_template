// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(
    Debug,
    generate_constructor::New,
    serde::Serialize,
    serde::Deserialize,
    optimal_memory_layout::OptimalMemoryLayout,
)]
pub struct SerdeStruct {
    one: crate::location_test_text::LocationTestText,
    three: crate::location_test_count::LocationTestCount,
    two: crate::location_test_flag::LocationTestFlag,
}

impl to_err_string::to_err_string::ToErrString for SerdeStruct {
    fn to_err_string(&self) -> to_err_string::error_text::ErrorText {
        to_err_string::error_text::ErrorText::try_from(format!("{self:?}"))
            .unwrap_or_else(to_err_string::error_text::ErrorText::from)
    }
}
