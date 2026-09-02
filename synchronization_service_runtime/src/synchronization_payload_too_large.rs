#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
)]
pub enum SynchronizationPayloadTooLarge {
    #[error("{}", std::any::type_name::<Self>())]
    TooLarge,
}

impl From<bounded_types::bounded_value_error::BoundedValueError>
    for SynchronizationPayloadTooLarge
{
    fn from(bounded_value_error: bounded_types::bounded_value_error::BoundedValueError) -> Self {
        let _: bounded_types::bounded_value_error::BoundedValueError = bounded_value_error;
        Self::TooLarge
    }
}
