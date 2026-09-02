#[derive(
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
)]
// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering)]
pub struct DbColumnContractSnapshot {
    #[constructor(order = 1)]
    data_type: crate::db_schema_text::DbSchemaText,
    #[constructor(order = 0)]
    name: crate::db_schema_text::DbSchemaText,
    #[constructor(order = 3)]
    has_server_default: crate::db_column_has_server_default::DbColumnHasServerDefault,
    #[constructor(order = 2)]
    nullable: crate::db_column_nullable::DbColumnNullable,
}
