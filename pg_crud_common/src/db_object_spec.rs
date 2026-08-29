#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering)]
pub struct DbObjectSpec {
    pub(super) definition: crate::db_static_schema_text::DbStaticSchemaText,
    pub(super) name: crate::db_static_schema_text::DbStaticSchemaText,
    pub(super) kind: crate::db_object_kind::DbObjectKind,
}

impl DbObjectSpec {
    #[must_use]
    pub const fn new(
        name: crate::db_static_schema_text::DbStaticSchemaText,
        kind: crate::db_object_kind::DbObjectKind,
        definition: crate::db_static_schema_text::DbStaticSchemaText,
    ) -> Self {
        Self {
            definition,
            name,
            kind,
        }
    }
}
