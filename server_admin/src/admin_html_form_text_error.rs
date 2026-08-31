#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum AdminHtmlFormTextError {
    #[error("{message}", message = constants_str::ADMIN_HTML_FORM_TEXT_TOO_LONG)]
    TooLong,
}
impl From<bounded_types::bounded_string_error::BoundedStringError> for AdminHtmlFormTextError {
    fn from(value: bounded_types::bounded_string_error::BoundedStringError) -> Self {
        let _: bounded_types::bounded_string_error::BoundedStringError = value;
        Self::TooLong
    }
}
