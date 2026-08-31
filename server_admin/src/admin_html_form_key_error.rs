#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum AdminHtmlFormKeyError {
    #[error("{message}", message = constants_str::ADMIN_HTML_FORM_KEY_TOO_LONG)]
    TooLong,
}
impl From<bounded_types::bounded_value_error::BoundedValueError> for AdminHtmlFormKeyError {
    fn from(_value: bounded_types::bounded_value_error::BoundedValueError) -> Self {
        Self::TooLong
    }
}
