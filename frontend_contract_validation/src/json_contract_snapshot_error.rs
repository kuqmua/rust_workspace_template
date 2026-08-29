#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum JsonContractSnapshotError {
    #[error("{}", constants_str::test_fixtures::JSON_SNAPSHOT_SERIALIZATION_ERROR)]
    Serialization,
    #[error("{}", constants_str::test_fixtures::JSON_SNAPSHOT_TOO_LONG_ERROR)]
    TooLong,
}
