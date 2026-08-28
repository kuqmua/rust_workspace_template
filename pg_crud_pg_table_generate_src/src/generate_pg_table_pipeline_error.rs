#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum GeneratePgTablePipelineError {
    #[error("{0}")]
    Build(super::SynGeneratePgTablePipelineError),
    #[error("{0}")]
    Parse(super::SynGeneratePgTablePipelineError),
    #[error("{0}")]
    Validate(super::SynGeneratePgTablePipelineError),
}
