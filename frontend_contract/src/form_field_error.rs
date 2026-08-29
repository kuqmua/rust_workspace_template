#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    PartialEq,
    Eq,
    generate_constructor::New,
)]
pub struct FormFieldError {
    error: crate::form_value_error::FormValueError,
    field: crate::contract_str::ContractStr,
}
impl FormFieldError {
    #[must_use]
    pub const fn error(&self) -> &crate::form_value_error::FormValueError {
        &self.error
    }
    #[must_use]
    pub const fn field(&self) -> crate::contract_str::ContractStr {
        self.field
    }
}
