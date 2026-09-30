#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub enum DbKeySpec {
    ForeignKey {
        columns: crate::db_static_schema_texts::DbStaticSchemaTexts,
        referenced_columns: crate::db_static_schema_texts::DbStaticSchemaTexts,
        referenced_table: crate::db_static_schema_text::DbStaticSchemaText,
        on_delete: crate::db_foreign_key_delete_action::DbForeignKeyDeleteAction,
    },
    PrimaryKey(crate::db_static_schema_texts::DbStaticSchemaTexts),
    Unique(crate::db_static_schema_texts::DbStaticSchemaTexts),
}
