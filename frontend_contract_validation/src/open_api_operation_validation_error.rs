#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::DebugDisplay, thiserror::Error,
)]
pub enum OpenApiOperationValidationError {
    DocumentSerialization(
        crate::serde_json_open_api_serialization_error::SerdeJsonOpenApiSerializationError,
    ),
    MissingContentType,
    MissingOperation,
    MissingResponseSchema,
    MissingResponseStatus,
    SecurityMismatch,
}
