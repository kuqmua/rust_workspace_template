#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "generate pg types config keeps declaration order aligned with generated layout or processing flow"
)]
#[derive(
    proc_macro_getters::Getters,
    Debug,
    serde::Deserialize,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub(super) struct GeneratePgTypesConfig {
    variant: crate::generate_pg_types_config_variant::GeneratePgTypesConfigVariant,
    pg_table_cols_write_into_file:
        macro_helpers::should_write_token_stream_into_file::ShouldWriteTokenStreamIntoFile,
    whole_write_into_file:
        macro_helpers::should_write_token_stream_into_file::ShouldWriteTokenStreamIntoFile,
    #[serde(default)]
    generate_secret_text: crate::generate_secret_text::GenerateSecretText,
}
impl GeneratePgTypesConfig {
    pub(super) fn into_parts(
        self,
    ) -> (
        crate::generate_pg_types_config_variant::GeneratePgTypesConfigVariant,
        macro_helpers::should_write_token_stream_into_file::ShouldWriteTokenStreamIntoFile,
        macro_helpers::should_write_token_stream_into_file::ShouldWriteTokenStreamIntoFile,
        crate::generate_secret_text::GenerateSecretText,
    ) {
        (
            self.variant,
            self.pg_table_cols_write_into_file,
            self.whole_write_into_file,
            self.generate_secret_text,
        )
    }
}
