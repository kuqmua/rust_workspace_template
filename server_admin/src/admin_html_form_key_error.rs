#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
#[error("{message}", message = constants_str::test_fixtures::ADMIN_HTML_FORM_KEY_TOO_LONG)]
pub(crate) struct AdminHtmlFormKeyError;
impl From<bounded_types::bounded_value_error::BoundedValueError> for AdminHtmlFormKeyError {
    fn from(_value: bounded_types::bounded_value_error::BoundedValueError) -> Self {
        Self
    }
}
