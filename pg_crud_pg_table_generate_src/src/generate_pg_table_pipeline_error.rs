#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum GeneratePgTablePipelineError {
    #[error("{0}")]
    Build(crate::pipeline::SynGeneratePgTablePipelineError),
    #[error("{0}")]
    Parse(crate::pipeline::SynGeneratePgTablePipelineError),
    #[error("{0}")]
    Validate(crate::pipeline::SynGeneratePgTablePipelineError),
}
