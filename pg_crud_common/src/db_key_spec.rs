#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub enum DbKeySpec {
    ForeignKey {
        columns: super::DbStaticSchemaTexts,
        referenced_columns: super::DbStaticSchemaTexts,
        referenced_table: super::DbStaticSchemaText,
    },
    PrimaryKey(super::DbStaticSchemaTexts),
    Unique(super::DbStaticSchemaTexts),
}
