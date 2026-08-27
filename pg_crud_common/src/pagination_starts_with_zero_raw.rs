#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "split owner modules expose representation only within the crate"
)]
#[derive(
    Debug, serde::Deserialize, schemars::JsonSchema, optimal_memory_layout::OptimalMemoryLayout,
)]
pub(crate) struct PaginationStartsWithZeroRaw {
    pub(crate) limit: crate::domain_types::PaginationLimit,
    pub(crate) offset: crate::domain_types::PaginationOffset,
}
