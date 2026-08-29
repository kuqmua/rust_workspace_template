#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
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
