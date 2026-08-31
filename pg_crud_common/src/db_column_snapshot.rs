#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, Ord, PartialEq, PartialOrd,
)]
pub struct DbColumnSnapshot {
    data_type: crate::db_schema_text::DbSchemaText,
    default: Option<crate::db_schema_text::DbSchemaText>,
    name: crate::db_schema_text::DbSchemaText,
    nullable: crate::db_column_nullable::DbColumnNullable,
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
