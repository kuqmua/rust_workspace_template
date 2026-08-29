#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum MigrationsSourceError {
    #[error(
        "{0}",
        constants_str::test_fixtures::MIGRATIONS_SOURCE_EXCEEDS_MAXIMUM_LENGTH
    )]
    TooLong,
}
