#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error(transparent)]
pub struct SerdeJsonGeneratePgTypesError(serde_json::Error);
