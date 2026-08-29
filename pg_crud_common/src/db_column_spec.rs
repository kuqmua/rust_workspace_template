#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering)]
pub struct DbColumnSpec {
    pub(super) data_type: crate::db_static_schema_text::DbStaticSchemaText,
    pub(super) name: crate::db_static_schema_text::DbStaticSchemaText,
    pub(super) has_server_default: crate::db_column_has_server_default::DbColumnHasServerDefault,
    pub(super) nullable: crate::db_column_nullable::DbColumnNullable,
}

impl DbColumnSpec {
    #[must_use]
    pub const fn new(
        name: crate::db_static_schema_text::DbStaticSchemaText,
        data_type: crate::db_static_schema_text::DbStaticSchemaText,
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
