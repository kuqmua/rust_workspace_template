use crate::openapi_validation::{
    OpenApiContractText, OpenApiContractTextError, SerdeJsonOpenApiSerializationError,
};

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::DebugDisplay, thiserror::Error,
)]
pub enum OpenApiValidationError {
    DocumentSerialization(SerdeJsonOpenApiSerializationError),
    MissingOperationId(OpenApiContractText, OpenApiContractText),
    MissingPaths,
    MissingSchemaReference(OpenApiContractText),
    MissingSchemas,
    OpenApiRouteMissing(OpenApiContractText, OpenApiContractText),
    OperationIdMismatch(
        OpenApiContractText,
        OpenApiContractText,
        OpenApiContractText,
        OpenApiContractText,
    ),
    RuntimeRouteMissing(OpenApiContractText, OpenApiContractText),
    TextTooLong(OpenApiContractTextError),
    UnusedSchema(OpenApiContractText),
}
