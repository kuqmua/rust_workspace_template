#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug)]
pub struct BuiltGenerateWhereFiltersModel {
    pub(super) config: super::ParsedGenerateWhereFiltersConfig,
    pub(super) contract_valid: crate::domain_types::spec::FilterSpecValid,
}
