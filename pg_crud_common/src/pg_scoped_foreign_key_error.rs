#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum PgScopedForeignKeyError {
    #[error(
        "{}",
        constants_str::test_fixtures::PG_SCOPED_FOREIGN_KEY_COLUMN_COUNT_MISMATCH
    )]
    ColumnCountMismatch,
    #[error(
        "{}",
        constants_str::test_fixtures::PG_SCOPED_FOREIGN_KEY_DUPLICATE_COLUMN
    )]
    DuplicateColumn,
    #[error(
        "{}",
        constants_str::test_fixtures::PG_SCOPED_FOREIGN_KEY_INVALID_COLUMN_COUNT
    )]
    InvalidColumnCount,
}
