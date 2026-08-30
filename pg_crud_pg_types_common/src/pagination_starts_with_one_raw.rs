#[derive(
    Debug, serde::Deserialize, schemars::JsonSchema, optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct PaginationStartsWithOneRaw {
    limit: crate::pagination_starts_with_one_value::PaginationStartsWithOneValue,
    offset: crate::pagination_starts_with_one_value::PaginationStartsWithOneValue,
}
impl PaginationStartsWithOneRaw {
    pub(super) const fn into_parts(
        self,
    ) -> (
        crate::pagination_starts_with_one_value::PaginationStartsWithOneValue,
        crate::pagination_starts_with_one_value::PaginationStartsWithOneValue,
    ) {
        (self.limit, self.offset)
    }
}
