pub trait PgColumnSchema {
    const HAS_SERVER_DEFAULT: bool;
    const NULLABLE: bool;
    fn data_type() -> crate::db_static_schema_text::DbStaticSchemaText;
}
