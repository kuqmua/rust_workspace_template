#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum BoundedJsonReadError {
    #[error("bounded content read failed")]
    Read(#[source] super::BoundedReadError),
    #[error("bounded content is not valid JSON")]
    SerdeJson(#[source] super::SerdeJsonError),
}
