#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use super::*;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub struct BuiltGeneratePgTypesModel {
    pub(super) config: GeneratePgTypesConfig,
    pub(super) entry_count: PgTypesModelEntryCount,
}
