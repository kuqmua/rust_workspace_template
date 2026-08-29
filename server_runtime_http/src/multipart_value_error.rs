#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum MultipartValueError {
    #[error("multipart name must not contain control characters")]
    ControlCharacter,
    #[error("multipart field name must not be empty")]
    EmptyFieldName,
    #[error("multipart file name must not be empty")]
    EmptyFileName,
    #[error("multipart value must not contain NUL")]
    Nul,
    #[error("multipart file name must not contain path components")]
    PathComponent,
    #[error("multipart value length {actual} exceeds its maximum")]
    TooLong {
        actual: crate::multipart_value_length::MultipartValueLength,
    },
}
