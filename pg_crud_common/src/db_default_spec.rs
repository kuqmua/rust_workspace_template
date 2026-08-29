#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    generate_constructor::New,
)]
pub struct DbDefaultSpec {
    pub(super) column: crate::db_static_schema_text::DbStaticSchemaText,
    pub(super) expression: crate::db_static_schema_text::DbStaticSchemaText,
}
