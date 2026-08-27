#[derive(
    Debug, serde::Deserialize, schemars::JsonSchema, optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct PaginationStartsWithOneRaw {
    pub(super) limit: super::PaginationStartsWithOneValue,
    pub(super) offset: super::PaginationStartsWithOneValue,
}
