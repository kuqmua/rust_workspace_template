pub trait DbTableSchema {
    fn columns() -> crate::db_column_specs::DbColumnSpecs;
    fn create_excluded_columns() -> crate::db_static_schema_texts::DbStaticSchemaTexts;
    fn keys() -> crate::db_key_specs::DbKeySpecs;
    fn primary_key_column() -> crate::db_static_schema_text::DbStaticSchemaText;
    fn read_excluded_columns() -> crate::db_static_schema_texts::DbStaticSchemaTexts;
    fn schema_table_text() -> crate::db_static_schema_text::DbStaticSchemaText;
}
