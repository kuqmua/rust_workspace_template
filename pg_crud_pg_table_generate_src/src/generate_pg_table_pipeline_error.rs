#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum GeneratePgTablePipelineError {
    #[error("{0}")]
    Build(crate::SynGeneratePgTablePipelineError),
    #[error("{0}")]
    Parse(crate::SynGeneratePgTablePipelineError),
    #[error("{0}")]
    Validate(crate::SynGeneratePgTablePipelineError),
}
