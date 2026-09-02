#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    PartialEq,
    Eq,
    proc_macro_getters::Getters,
    proc_macro_new::New,
)]
pub struct FormFieldError {
    error: crate::form_value_error::FormValueError,
    field: crate::contract_str::ContractStr,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_generated_getters_expose_form_field_error_values() {
        let error = crate::form_value_error::FormValueError::default();
        let field = crate::contract_str::ContractStr::from(constants_str::FIELD);
        let value = super::FormFieldError::new(error.clone(), field);
        assert_eq!(value.get_error(), &error);
        assert_eq!(value.get_field(), &field);
    }
}
