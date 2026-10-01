#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum BoundedJsonReadError {
    #[error("bounded content read failed")]
    Read(#[source] crate::bounded_read_error::BoundedReadError),
    #[error("bounded content is not valid JSON")]
    SerdeJson(#[source] crate::serde_json_error::SerdeJsonError),
}

impl BoundedJsonReadError {
    pub(crate) fn from_string_bounds(
        bounded_string_error: bounded_types::bounded_string_error::BoundedStringError,
    ) -> Self {
        match bounded_string_error {
            bounded_types::bounded_string_error::BoundedStringError::AboveMaximum {
                maximum_length,
                ..
            }
            | bounded_types::bounded_string_error::BoundedStringError::BelowMinimum {
                minimum_length: maximum_length,
                ..
            } => Self::Read(
                crate::bounded_read_error::BoundedReadError::ExceedsMaximum {
                    maximum_bytes: crate::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(
                        maximum_length.get(),
                    ),
                },
            ),
        }
    }
}
