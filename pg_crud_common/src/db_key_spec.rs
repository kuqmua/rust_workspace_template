#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub enum DbKeySpec {
    ForeignKey {
        columns: crate::db_static_schema_texts::DbStaticSchemaTexts,
        referenced_columns: crate::db_static_schema_texts::DbStaticSchemaTexts,
        referenced_table: crate::db_static_schema_text::DbStaticSchemaText,
    },
    PrimaryKey(crate::db_static_schema_texts::DbStaticSchemaTexts),
    Unique(crate::db_static_schema_texts::DbStaticSchemaTexts),
}
