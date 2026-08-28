#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, Ord, PartialEq, PartialOrd,
)]
pub enum DbKeyContractSnapshot {
    ForeignKey {
        columns: super::DbSchemaTexts,
        referenced_columns: super::DbSchemaTexts,
        referenced_table: super::DbSchemaText,
    },
    PrimaryKey(super::DbSchemaTexts),
    Unique(super::DbSchemaTexts),
}
