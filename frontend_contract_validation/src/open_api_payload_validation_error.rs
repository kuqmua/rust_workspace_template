#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_newtype_debug_display::DebugDisplay,
    thiserror::Error,
)]
pub enum OpenApiPayloadValidationError {
    Mismatch(crate::open_api_schema_mismatch::OpenApiSchemaMismatch),
}
