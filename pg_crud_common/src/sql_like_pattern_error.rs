#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("{}", constants_str::SQL_LIKE_PATTERN_EXCEEDS_MAXIMUM_LENGTH)]
pub struct SqlLikePatternError;
