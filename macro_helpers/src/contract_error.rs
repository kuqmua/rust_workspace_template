#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum ContractError {
    #[error("fixture JSON deserialization failed: {0}")]
    DeserializeFixture(crate::macro_serde_json_error::MacroSerdeJsonError),
    #[error("round-trip JSON deserialization failed: {0}")]
    DeserializeRoundTrip(crate::macro_serde_json_error::MacroSerdeJsonError),
    #[error("JSON serialization failed: {0}")]
    Serialize(crate::macro_serde_json_error::MacroSerdeJsonError),
    #[error("round-trip value differs from fixture value")]
    ValueMismatch,
}
