#[derive(
    Debug,
    serde::Deserialize,
    schemars::JsonSchema,
    optimal_memory_layout::OptimalMemoryLayout,
    generate_accessor::Getters,
)]
#[getters(get_mut)]
pub(crate) struct PaginationStartsWithZeroRaw {
    limit: crate::domain_types::PaginationLimit,
    offset: crate::domain_types::PaginationOffset,
}
