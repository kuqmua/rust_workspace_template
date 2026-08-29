#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum PgOperationalLimitError {
    #[error(
        "{}",
        constants_str::test_fixtures::PG_OPERATIONAL_LIMIT_BELOW_CURRENT_USAGE
    )]
    BelowCurrentUsage,
    #[error(
        "{}",
        constants_str::test_fixtures::PG_OPERATIONAL_LIMIT_MUST_BE_GREATER_THAN_ZERO
    )]
    ZeroLimit,
}
