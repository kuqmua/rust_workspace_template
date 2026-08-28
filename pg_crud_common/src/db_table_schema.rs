pub trait DbTableSchema {
    fn columns() -> super::DbColumnSpecs;
    fn create_excluded_columns() -> super::DbStaticSchemaTexts;
    fn keys() -> super::DbKeySpecs;
    fn primary_key_column() -> super::DbStaticSchemaText;
    fn read_excluded_columns() -> super::DbStaticSchemaTexts;
    fn schema_table_text() -> super::DbStaticSchemaText;
}
