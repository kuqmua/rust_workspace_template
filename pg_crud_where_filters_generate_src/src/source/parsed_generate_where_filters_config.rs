#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(Clone, Copy, Debug, serde::Deserialize, optimal_memory_layout::OptimalMemoryLayout)]
pub struct ParsedGenerateWhereFiltersConfig {
    pub(super) pg_types_write_into_file:
        macro_helpers::domain_types::ts_writer::ShouldWriteTokenStreamIntoFile,
    pub(super) whole_write_into_file:
        macro_helpers::domain_types::ts_writer::ShouldWriteTokenStreamIntoFile,
}
