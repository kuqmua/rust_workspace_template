use super::*;

#[derive(Debug, serde::Deserialize, optimal_memory_layout::OptimalMemoryLayout)]
pub(super) enum GeneratePgTypesConfigVariant {
    All,
    Concrete(GeneratePgTypeRecords),
    Subset(GeneratePgTypes),
}
