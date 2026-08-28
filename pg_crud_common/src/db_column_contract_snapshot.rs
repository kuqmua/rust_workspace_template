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
    pub(super) data_type: super::DbSchemaText,
    pub(super) name: super::DbSchemaText,
    pub(super) has_server_default: super::DbColumnHasServerDefault,
    pub(super) nullable: super::DbColumnNullable,
}

impl DbColumnContractSnapshot {
    #[must_use]
    pub const fn new(
        name: super::DbSchemaText,
        data_type: super::DbSchemaText,
        nullable: super::DbColumnNullable,
        has_server_default: super::DbColumnHasServerDefault,
    ) -> Self {
        Self {
            data_type,
            name,
            has_server_default,
            nullable,
        }
    }
}
