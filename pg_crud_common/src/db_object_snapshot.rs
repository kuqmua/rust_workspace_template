#[derive(
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
)]
#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "lint suppression is required here"
)]
pub struct DbObjectSnapshot {
    #[constructor(order = 2)]
    definition: crate::db_schema_text::DbSchemaText,
    #[constructor(order = 0)]
    name: crate::db_schema_text::DbSchemaText,
    #[constructor(order = 1)]
    kind: crate::db_object_kind::DbObjectKind,
}
