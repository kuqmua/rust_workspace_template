#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum StdAdminHtmlSelectedError {
    #[error("administrator HTML form contains too many selected fields")]
    TooMany,
}
impl From<bounded_types::bounded_value_error::BoundedValueError> for StdAdminHtmlSelectedError {
    fn from(value: bounded_types::bounded_value_error::BoundedValueError) -> Self {
        let _: bounded_types::bounded_value_error::BoundedValueError = value;
        Self::TooMany
    }
}
