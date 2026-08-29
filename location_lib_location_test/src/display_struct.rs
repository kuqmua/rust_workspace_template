#[derive(Debug, optimal_memory_layout::OptimalMemoryLayout)]
pub struct DisplayStruct {
    pub display: crate::location_test_text::LocationTestText,
    pub something: crate::location_test_flag::LocationTestFlag,
}

impl to_err_string::to_err_string::ToErrString for DisplayStruct {
    fn to_err_string(&self) -> to_err_string::error_text::ErrorText {
        to_err_string::error_text::ErrorText::try_from(format!("{self:?}"))
            .unwrap_or_else(to_err_string::error_text::ErrorText::from)
    }
}
