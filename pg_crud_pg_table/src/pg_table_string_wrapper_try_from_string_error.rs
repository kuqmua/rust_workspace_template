#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    thiserror::Error,
)]
pub enum PgTableStringWrapperTryFromStringError {
    #[error("pg table string wrapper length {len} exceeds maximum {max}")]
    TooLong { len: usize, max: usize },
}
impl to_err_string::to_err_string::ToErrString for PgTableStringWrapperTryFromStringError {
    fn to_err_string(&self) -> to_err_string::error_text::ErrorText {
        to_err_string::error_text::ErrorText::try_from(self.to_string())
            .unwrap_or_else(to_err_string::error_text::ErrorText::from)
    }
}
