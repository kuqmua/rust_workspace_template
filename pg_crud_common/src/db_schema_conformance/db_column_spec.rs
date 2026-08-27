#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
#[allow(clippy::arbitrary_source_item_ordering)]
pub struct DbColumnSpec {
    pub(super) data_type: super::DbStaticSchemaText,
    pub(super) name: super::DbStaticSchemaText,
    pub(super) has_server_default: super::DbColumnHasServerDefault,
    pub(super) nullable: super::DbColumnNullable,
}

impl DbColumnSpec {
    #[must_use]
    pub const fn new(
        name: super::DbStaticSchemaText,
        data_type: super::DbStaticSchemaText,
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
