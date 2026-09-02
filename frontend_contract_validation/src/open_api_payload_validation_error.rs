#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype::DebugDisplay,
    thiserror::Error,
)]
pub enum OpenApiPayloadValidationError {
    DocumentSerialization(
        crate::serde_json_open_api_serialization_error::SerdeJsonOpenApiSerializationError,
    ),
    Mismatch(crate::open_api_schema_mismatch::OpenApiSchemaMismatch),
    PayloadSerialization(
        crate::serde_json_open_api_serialization_error::SerdeJsonOpenApiSerializationError,
    ),
    SchemaSerialization(
        crate::serde_json_open_api_serialization_error::SerdeJsonOpenApiSerializationError,
    ),
}
