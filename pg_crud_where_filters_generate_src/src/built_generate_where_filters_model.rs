#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug)]
pub struct BuiltGenerateWhereFiltersModel {
    pub(super) config: crate::source::ParsedGenerateWhereFiltersConfig,
    pub(super) contract_valid: crate::spec::FilterSpecValid,
}
