#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("frontend contract body exceeds its maximum byte length")]
pub struct FrontendContractBodyError;

impl From<bounded_types::bounded_value_error::BoundedValueError> for FrontendContractBodyError {
    fn from(_value: bounded_types::bounded_value_error::BoundedValueError) -> Self {
        Self
    }
}
