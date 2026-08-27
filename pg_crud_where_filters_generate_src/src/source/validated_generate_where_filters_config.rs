#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub struct ValidatedGenerateWhereFiltersConfig(pub(super) super::ParsedGenerateWhereFiltersConfig);
