#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(
    Debug, serde::Deserialize, schemars::JsonSchema, optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct PaginationStartsWithOneRaw {
    pub(super) limit: crate::domain_types::PaginationStartsWithOneValue,
    pub(super) offset: crate::domain_types::PaginationStartsWithOneValue,
}
