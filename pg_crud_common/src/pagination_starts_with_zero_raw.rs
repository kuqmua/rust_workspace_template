#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "split owner modules expose representation only within the crate"
)]
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
