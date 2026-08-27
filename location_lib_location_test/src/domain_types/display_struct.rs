#[derive(Debug, optimal_memory_layout::OptimalMemoryLayout)]
pub struct DisplayStruct {
    pub display: super::LocationTestText,
    pub something: super::LocationTestFlag,
}

impl to_err_string::domain_types::ToErrString for DisplayStruct {
    fn to_err_string(&self) -> to_err_string::domain_types::ErrorText {
        to_err_string::domain_types::ErrorText::try_from(format!("{self:?}"))
            .unwrap_or_else(to_err_string::domain_types::ErrorText::from)
    }
}
