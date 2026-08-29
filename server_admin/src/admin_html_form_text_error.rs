#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
#[error("{message}", message = constants_str::test_fixtures::ADMIN_HTML_FORM_TEXT_TOO_LONG)]
pub(crate) struct AdminHtmlFormTextError;
impl From<bounded_types::bounded_value_error::BoundedValueError> for AdminHtmlFormTextError {
    fn from(_value: bounded_types::bounded_value_error::BoundedValueError) -> Self {
        Self
    }
}
