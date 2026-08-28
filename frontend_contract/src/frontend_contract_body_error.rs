#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("frontend contract body exceeds its maximum byte length")]
pub struct FrontendContractBodyError;

impl From<bounded_types::domain_types::BoundedValueError> for FrontendContractBodyError {
    fn from(_value: bounded_types::domain_types::BoundedValueError) -> Self {
        Self
    }
}
