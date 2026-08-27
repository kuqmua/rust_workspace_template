#[derive(Clone, Copy, Debug, serde::Deserialize, optimal_memory_layout::OptimalMemoryLayout)]
pub struct ParsedGenerateWhereFiltersConfig {
    pub(super) pg_types_write_into_file:
        macro_helpers::domain_types::ts_writer::ShouldWriteTokenStreamIntoFile,
    pub(super) whole_write_into_file:
        macro_helpers::domain_types::ts_writer::ShouldWriteTokenStreamIntoFile,
}
