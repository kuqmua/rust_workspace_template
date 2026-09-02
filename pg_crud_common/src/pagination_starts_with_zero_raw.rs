#[derive(
    Debug,
    serde::Deserialize,
    schemars::JsonSchema,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_getters::Getters,
)]
#[getters(get_mut)]
pub(crate) struct PaginationStartsWithZeroRaw {
    limit: crate::pagination_limit::PaginationLimit,
    offset: crate::pagination_offset::PaginationOffset,
}
