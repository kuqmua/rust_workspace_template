#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, Ord, PartialEq, PartialOrd,
)]
// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering)]
pub struct DbColumnContractSnapshot {
    pub(super) data_type: crate::db_schema_text::DbSchemaText,
    pub(super) name: crate::db_schema_text::DbSchemaText,
    pub(super) has_server_default: crate::db_column_has_server_default::DbColumnHasServerDefault,
    pub(super) nullable: crate::db_column_nullable::DbColumnNullable,
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
