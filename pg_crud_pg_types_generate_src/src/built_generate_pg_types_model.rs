#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub struct BuiltGeneratePgTypesModel {
    pub(super) config: crate::generate_pg_types_config::GeneratePgTypesConfig,
    pub(super) entry_count: crate::pg_types_model_entry_count::PgTypesModelEntryCount,
}
