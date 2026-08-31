#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, Ord, PartialEq, PartialOrd,
)]
// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering)]
pub struct DbObjectSnapshot {
    definition: crate::db_schema_text::DbSchemaText,
    name: crate::db_schema_text::DbSchemaText,
    kind: crate::db_object_kind::DbObjectKind,
}

impl DbObjectSnapshot {
    #[must_use]
    pub const fn new(
        name: crate::db_schema_text::DbSchemaText,
        kind: crate::db_object_kind::DbObjectKind,
        definition: crate::db_schema_text::DbSchemaText,
    ) -> Self {
        Self {
            definition,
            name,
            kind,
        }
    }
}
