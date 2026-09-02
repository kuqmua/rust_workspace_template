#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum GeneratePgTypesPipelineError {
    #[error("{0}")]
    Parse(crate::serde_json_generate_pg_types_error::SerdeJsonGeneratePgTypesError),
}
