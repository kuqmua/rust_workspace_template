#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum SqlLikePatternError {
    #[error(
        "{}",
        constants_str::test_fixtures::SQL_LIKE_PATTERN_EXCEEDS_MAXIMUM_LENGTH
    )]
    TooLong,
}
