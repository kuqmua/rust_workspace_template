#[derive(
    proc_macro_getters::Getters,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    proc_macro_new::New,
)]
pub struct DbDefaultSpec {
    column: crate::db_static_schema_text::DbStaticSchemaText,
    expression: crate::db_static_schema_text::DbStaticSchemaText,
}
