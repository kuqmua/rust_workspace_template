#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug)]
pub struct BuiltGenerateWhereFiltersModel {
    pub(super) config:
        crate::parsed_generate_where_filters_config::ParsedGenerateWhereFiltersConfig,
    pub(super) contract_valid: crate::filter_spec_valid::FilterSpecValid,
}
