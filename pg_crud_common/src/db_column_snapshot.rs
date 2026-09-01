#[derive(
    generate_constructor::New,
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub struct DbColumnSnapshot {
    #[constructor(order = 1)]
    data_type: crate::db_schema_text::DbSchemaText,
    #[constructor(order = 3)]
    default: Option<crate::db_schema_text::DbSchemaText>,
    #[constructor(order = 0)]
    name: crate::db_schema_text::DbSchemaText,
    #[constructor(order = 2)]
    nullable: crate::db_column_nullable::DbColumnNullable,
}
