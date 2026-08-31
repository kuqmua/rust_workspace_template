#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error("{0}")]
pub struct SerdeJsonGeoJsonError(serde_json::Error);
