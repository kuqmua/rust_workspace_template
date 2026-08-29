#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum DataInvariantViolation {
    #[error("{}", constants_str::test_fixtures::BULK_FAILURE_CHANGED_STATE)]
    BulkFailureChangedState,
    #[error("{}", constants_str::test_fixtures::BULK_MUTATION_MUST_FAIL)]
    BulkMutationMustFail,
    #[error(
        "{}",
        constants_str::test_fixtures::MIGRATION_SECOND_RUN_CHANGED_SCHEMA
    )]
    MigrationSecondRunChangedSchema,
    #[error("{}", constants_str::test_fixtures::PAGINATION_ITEMS_OVERLAP)]
    PaginationItemsOverlap,
    #[error("{}", constants_str::test_fixtures::PAGINATION_TOTAL_CHANGED)]
    PaginationTotalChanged,
}
