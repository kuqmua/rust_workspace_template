#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum CursorDecodeError {
    #[error("{message}", message = constants_str::CURSOR_FORMAT_IS_INVALID)]
    InvalidFormat,
    #[error("{message}", message = constants_str::CURSOR_PAYLOAD_IS_INVALID)]
    InvalidPayload,
    #[error("{message}", message = constants_str::CURSOR_SIGNATURE_IS_INVALID)]
    InvalidSignature,
    #[error("{message}", message = constants_str::CURSOR_SIGNING_KEY_IS_INVALID)]
    InvalidSigningKey,
    #[error("{message}", message = constants_str::CURSOR_EXCEEDS_MAXIMUM_LENGTH)]
    MaximumLengthExceeded,
}
