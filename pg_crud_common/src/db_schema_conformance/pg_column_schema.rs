pub trait PgColumnSchema {
    const HAS_SERVER_DEFAULT: bool;
    const NULLABLE: bool;
    fn data_type() -> super::DbStaticSchemaText;
}
