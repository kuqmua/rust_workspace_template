#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
)]
pub(crate) enum AdminAuthCollectionError {
    #[error("{message}", message = constants_str::ADMIN_DIAGNOSTIC_ADMINISTRATOR_AUTHORIZATION_COLLECTION_EXCEEDS_MAXIMUM_LENGTH)]
    TooLarge,
}

impl From<bounded_types::bounded_value_error::BoundedValueError> for AdminAuthCollectionError {
    fn from(value: bounded_types::bounded_value_error::BoundedValueError) -> Self {
        let _: bounded_types::bounded_value_error::BoundedValueError = value;
        Self::TooLarge
    }
}
