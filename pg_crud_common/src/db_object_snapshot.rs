#[derive(
    generate_constructor::New,
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
)]
// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering)]
pub struct DbObjectSnapshot {
    #[constructor(order = 2)]
    definition: crate::db_schema_text::DbSchemaText,
    #[constructor(order = 0)]
    name: crate::db_schema_text::DbSchemaText,
    #[constructor(order = 1)]
    kind: crate::db_object_kind::DbObjectKind,
}
