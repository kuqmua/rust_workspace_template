use super::*;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum GeneratePgTypesPipelineError {
    #[error("{0}")]
    Parse(SerdeJsonGeneratePgTypesError),
}
