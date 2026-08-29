#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]

// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, serde::Deserialize, optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct GeneratePgTypesConfig {
    pub(super) variant: crate::generate_pg_types_config_variant::GeneratePgTypesConfigVariant,
    pub(super) pg_table_cols_write_into_file:
        macro_helpers::should_write_token_stream_into_file::ShouldWriteTokenStreamIntoFile,
    pub(super) whole_write_into_file:
        macro_helpers::should_write_token_stream_into_file::ShouldWriteTokenStreamIntoFile,
    #[serde(default)]
    pub(super) generate_secret_text: crate::generate_secret_text::GenerateSecretText,
}
