#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
#[error("{message}", message = constants_str::ADMIN_HTML_FORM_KEY_TOO_LONG)]
pub(super) struct AdminHtmlFormKeyError;
impl From<bounded_types::domain_types::BoundedValueError> for AdminHtmlFormKeyError {
    fn from(_value: bounded_types::domain_types::BoundedValueError) -> Self {
        Self
    }
}
