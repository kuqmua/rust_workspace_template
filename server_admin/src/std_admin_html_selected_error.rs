#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
#[error("administrator HTML form contains too many selected fields")]
pub(crate) struct StdAdminHtmlSelectedError;
impl From<bounded_types::BoundedValueError> for StdAdminHtmlSelectedError {
    fn from(_value: bounded_types::BoundedValueError) -> Self {
        Self
    }
}
