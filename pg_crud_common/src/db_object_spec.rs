#[derive(
    proc_macro_getters::Getters,
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
)]
// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering)]
pub struct DbObjectSpec {
    #[constructor(order = 2)]
    definition: crate::db_static_schema_text::DbStaticSchemaText,
    #[constructor(order = 0)]
    name: crate::db_static_schema_text::DbStaticSchemaText,
    #[constructor(order = 1)]
    kind: crate::db_object_kind::DbObjectKind,
}
