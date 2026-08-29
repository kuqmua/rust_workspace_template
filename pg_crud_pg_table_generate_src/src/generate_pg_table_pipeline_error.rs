#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum GeneratePgTablePipelineError {
    #[error("{0}")]
    Build(crate::syn_generate_pg_table_pipeline_error::SynGeneratePgTablePipelineError),
    #[error("{0}")]
    Parse(crate::syn_generate_pg_table_pipeline_error::SynGeneratePgTablePipelineError),
    #[error("{0}")]
    Validate(crate::syn_generate_pg_table_pipeline_error::SynGeneratePgTablePipelineError),
}
