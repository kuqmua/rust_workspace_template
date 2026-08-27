use super::{ContractStr, FormValueError};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, PartialEq, Eq)]
pub struct FormFieldError {
    error: FormValueError,
    field: ContractStr,
}
impl FormFieldError {
    #[must_use]
    pub const fn new(error: FormValueError, field: ContractStr) -> Self {
        Self { error, field }
    }
    #[must_use]
    pub const fn error(&self) -> &FormValueError {
        &self.error
    }
    #[must_use]
    pub const fn field(&self) -> ContractStr {
        self.field
    }
}
