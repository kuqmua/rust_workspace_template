use super::openapi_validation::SerdeJsonOpenApiSerializationError;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::DebugDisplay, thiserror::Error,
)]
pub enum OpenApiOperationValidationError {
    DocumentSerialization(SerdeJsonOpenApiSerializationError),
    MissingContentType,
    MissingOperation,
    MissingResponseSchema,
    MissingResponseStatus,
    SecurityMismatch,
}
