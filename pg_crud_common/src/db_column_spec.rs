#[derive(
    proc_macro_getters::Getters,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
)]
#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "db column spec keeps declaration order aligned with generated layout or processing flow"
)]
#[derive(proc_macro_new::New)]
pub struct DbColumnSpec {
    #[constructor(order = 1)]
    data_type: crate::db_static_schema_text::DbStaticSchemaText,
    #[constructor(order = 0)]
    name: crate::db_static_schema_text::DbStaticSchemaText,
    #[constructor(order = 3)]
    has_server_default: crate::db_column_has_server_default::DbColumnHasServerDefault,
    #[constructor(order = 2)]
    nullable: crate::db_column_nullable::DbColumnNullable,
}
