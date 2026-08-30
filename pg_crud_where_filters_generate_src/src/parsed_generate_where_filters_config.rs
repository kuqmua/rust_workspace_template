#[derive(Clone, Copy, Debug, serde::Deserialize, optimal_memory_layout::OptimalMemoryLayout)]
pub struct ParsedGenerateWhereFiltersConfig {
    pg_types_write_into_file:
        macro_helpers::should_write_token_stream_into_file::ShouldWriteTokenStreamIntoFile,
    whole_write_into_file:
        macro_helpers::should_write_token_stream_into_file::ShouldWriteTokenStreamIntoFile,
}

impl ParsedGenerateWhereFiltersConfig {
    #[must_use]
    pub(crate) const fn into_parts(
        self,
    ) -> (
        macro_helpers::should_write_token_stream_into_file::ShouldWriteTokenStreamIntoFile,
        macro_helpers::should_write_token_stream_into_file::ShouldWriteTokenStreamIntoFile,
    ) {
        (self.pg_types_write_into_file, self.whole_write_into_file)
    }
}
