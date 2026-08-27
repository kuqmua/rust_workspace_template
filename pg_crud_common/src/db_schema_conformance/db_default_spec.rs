#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct DbDefaultSpec {
    pub(super) column: super::DbStaticSchemaText,
    pub(super) expression: super::DbStaticSchemaText,
}

impl DbDefaultSpec {
    #[must_use]
    pub const fn new(
        column: super::DbStaticSchemaText,
        expression: super::DbStaticSchemaText,
    ) -> Self {
        Self { column, expression }
    }
}
