#[derive(
    generate_accessor::Getters,
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    generate_constructor::New,
)]
pub struct DbDefaultSpec {
    column: crate::db_static_schema_text::DbStaticSchemaText,
    expression: crate::db_static_schema_text::DbStaticSchemaText,
}
