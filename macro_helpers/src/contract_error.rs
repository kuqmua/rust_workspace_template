#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum ContractError {
    #[error("fixture JSON deserialization failed: {0}")]
    DeserializeFixture(crate::serde_json_error::SerdeJsonError),
    #[error("round-trip JSON deserialization failed: {0}")]
    DeserializeRoundTrip(crate::serde_json_error::SerdeJsonError),
    #[error("JSON serialization failed: {0}")]
    Serialize(crate::serde_json_error::SerdeJsonError),
    #[error("round-trip value differs from fixture value")]
    ValueMismatch,
}
