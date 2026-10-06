#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    thiserror::Error,
    utoipa::ToSchema,
)]
pub enum PgCrudStringWrapperTryFromStringError {
    #[error("string wrapper length {len} exceeds maximum {max}")]
    TooLong { len: usize, max: usize },
}

impl to_err_string::to_err_string::ToErrString for PgCrudStringWrapperTryFromStringError {
    fn to_err_string(&self) -> to_err_string::error_text::ErrorText {
        to_err_string::error_text::ErrorText::try_from(self.to_string())
            .unwrap_or_else(to_err_string::error_text::ErrorText::from)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_string_wrapper_error_preserves_error_text_and_serialized_lengths() {
        [(1usize, 0usize), (65_535usize, 4096usize), (1_048_577usize, 1_048_576usize)]
            .into_iter()
            .fold((), |(), (len, max)| {
                let error = crate::pg_crud_string_wrapper_try_from_string_error::PgCrudStringWrapperTryFromStringError::TooLong { len, max };
                let error_text = to_err_string::to_err_string::ToErrString::to_err_string(&error);
                assert_eq!(error_text.as_ref(), error.to_string());
                let expected = serde_json::json!({
                    stringify!(TooLong): { stringify!(len): len, stringify!(max): max }
                });
                assert!(serde_json::to_value(error).is_ok_and(|value| value == expected));
                assert!(serde_json::from_value::<crate::pg_crud_string_wrapper_try_from_string_error::PgCrudStringWrapperTryFromStringError>(expected)
                    .is_ok_and(|decoded| decoded == error));
            });
    }
}
