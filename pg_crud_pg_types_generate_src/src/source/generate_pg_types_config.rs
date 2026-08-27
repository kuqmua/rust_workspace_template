#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use super::*;

// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, serde::Deserialize, optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct GeneratePgTypesConfig {
    pub(super) variant: GeneratePgTypesConfigVariant,
    pub(super) pg_table_cols_write_into_file:
        macro_helpers::domain_types::ts_writer::ShouldWriteTokenStreamIntoFile,
    pub(super) whole_write_into_file:
        macro_helpers::domain_types::ts_writer::ShouldWriteTokenStreamIntoFile,
    #[serde(default)]
    pub(super) generate_secret_text: GenerateSecretText,
}
