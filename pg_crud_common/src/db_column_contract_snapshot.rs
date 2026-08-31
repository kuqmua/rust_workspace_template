#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, Ord, PartialEq, PartialOrd,
)]
// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering)]
pub struct DbColumnContractSnapshot {
    data_type: crate::db_schema_text::DbSchemaText,
    name: crate::db_schema_text::DbSchemaText,
    has_server_default: crate::db_column_has_server_default::DbColumnHasServerDefault,
    nullable: crate::db_column_nullable::DbColumnNullable,
}

impl DbColumnContractSnapshot {
    #[must_use]
    pub const fn new(
        name: crate::db_schema_text::DbSchemaText,
        data_type: crate::db_schema_text::DbSchemaText,
        nullable: crate::db_column_nullable::DbColumnNullable,
        has_server_default: crate::db_column_has_server_default::DbColumnHasServerDefault,
    ) -> Self {
        Self {
            data_type,
            name,
            has_server_default,
            nullable,
        }
    }
}
