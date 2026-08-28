use crate::openapi_validation::{OpenApiSchemaMismatch, SerdeJsonOpenApiSerializationError};

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::DebugDisplay, thiserror::Error,
)]
pub enum OpenApiPayloadValidationError {
    DocumentSerialization(SerdeJsonOpenApiSerializationError),
    Mismatch(OpenApiSchemaMismatch),
    PayloadSerialization(SerdeJsonOpenApiSerializationError),
    SchemaSerialization(SerdeJsonOpenApiSerializationError),
}
