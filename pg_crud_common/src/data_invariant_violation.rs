#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum DataInvariantViolation {
    #[error("{}", constants_str::BULK_FAILURE_CHANGED_STATE)]
    BulkFailureChangedState,
    #[error("{}", constants_str::BULK_MUTATION_MUST_FAIL)]
    BulkMutationMustFail,
    #[error("{}", constants_str::MIGRATION_SECOND_RUN_CHANGED_SCHEMA)]
    MigrationSecondRunChangedSchema,
    #[error("{}", constants_str::PAGINATION_ITEMS_OVERLAP)]
    PaginationItemsOverlap,
    #[error("{}", constants_str::PAGINATION_TOTAL_CHANGED)]
    PaginationTotalChanged,
}
