#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, Ord, PartialEq, PartialOrd,
)]
pub enum DbKeyContractSnapshot {
    ForeignKey {
        columns: crate::db_schema_texts::DbSchemaTexts,
        referenced_columns: crate::db_schema_texts::DbSchemaTexts,
        referenced_table: crate::db_schema_text::DbSchemaText,
    },
    PrimaryKey(crate::db_schema_texts::DbSchemaTexts),
    Unique(crate::db_schema_texts::DbSchemaTexts),
}
