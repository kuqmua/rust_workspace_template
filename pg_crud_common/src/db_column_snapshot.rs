#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, Ord, PartialEq, PartialOrd,
)]
pub struct DbColumnSnapshot {
    pub(super) data_type: super::DbSchemaText,
    pub(super) default: Option<super::DbSchemaText>,
    pub(super) name: super::DbSchemaText,
    pub(super) nullable: super::DbColumnNullable,
}

impl DbColumnSnapshot {
    #[must_use]
    pub const fn new(
        name: super::DbSchemaText,
        data_type: super::DbSchemaText,
        nullable: super::DbColumnNullable,
        default: Option<super::DbSchemaText>,
    ) -> Self {
        Self {
            data_type,
            default,
            name,
            nullable,
        }
    }
}
