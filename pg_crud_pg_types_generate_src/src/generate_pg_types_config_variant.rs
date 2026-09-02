#[derive(Debug, serde::Deserialize, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub(super) enum GeneratePgTypesConfigVariant {
    All,
    Concrete(crate::generate_pg_type_records::GeneratePgTypeRecords),
    Subset(crate::generate_pg_types::GeneratePgTypes),
}
