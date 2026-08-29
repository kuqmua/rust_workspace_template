#[derive(Debug, serde::Deserialize, optimal_memory_layout::OptimalMemoryLayout)]
pub(super) enum GeneratePgTypesConfigVariant {
    All,
    Concrete(crate::generate_pg_type_records::GeneratePgTypeRecords),
    Subset(crate::generate_pg_types::GeneratePgTypes),
}
