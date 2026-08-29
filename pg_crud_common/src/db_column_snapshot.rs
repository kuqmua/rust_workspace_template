#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, Ord, PartialEq, PartialOrd,
)]
pub struct DbColumnSnapshot {
    pub(super) data_type: crate::db_schema_text::DbSchemaText,
    pub(super) default: Option<crate::db_schema_text::DbSchemaText>,
    pub(super) name: crate::db_schema_text::DbSchemaText,
    pub(super) nullable: crate::db_column_nullable::DbColumnNullable,
}

impl DbColumnSnapshot {
    #[must_use]
    pub const fn new(
        name: crate::db_schema_text::DbSchemaText,
        data_type: crate::db_schema_text::DbSchemaText,
        nullable: crate::db_column_nullable::DbColumnNullable,
        default: Option<crate::db_schema_text::DbSchemaText>,
    ) -> Self {
        Self {
            data_type,
            default,
            name,
            nullable,
        }
    }
}
