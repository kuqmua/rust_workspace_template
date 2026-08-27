#[derive(
    Debug, serde::Deserialize, schemars::JsonSchema, optimal_memory_layout::OptimalMemoryLayout,
)]
pub(crate) struct PaginationStartsWithZeroRaw {
    pub(crate) limit: crate::domain_types::PaginationLimit,
    pub(crate) offset: crate::domain_types::PaginationOffset,
}
