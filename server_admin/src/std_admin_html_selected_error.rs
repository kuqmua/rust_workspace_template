#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum StdAdminHtmlSelectedError {
    #[error("{message}", message = constants_str::ADMIN_DIAGNOSTIC_ADMINISTRATOR_HTML_FORM_CONTAINS_TOO_MANY_SELECTED_FIELDS)]
    TooMany,
}
impl From<bounded_types::bounded_value_error::BoundedValueError> for StdAdminHtmlSelectedError {
    fn from(value: bounded_types::bounded_value_error::BoundedValueError) -> Self {
        let _: bounded_types::bounded_value_error::BoundedValueError = value;
        Self::TooMany
    }
}
