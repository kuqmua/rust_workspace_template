#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
#[error("{message}", message = constants_str::ADMIN_HTML_FORM_TEXT_TOO_LONG)]
pub(super) struct AdminHtmlFormTextError;
impl From<bounded_types::domain_types::BoundedValueError> for AdminHtmlFormTextError {
    fn from(_value: bounded_types::domain_types::BoundedValueError) -> Self {
        Self
    }
}
