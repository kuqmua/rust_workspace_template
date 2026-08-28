#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, Ord, PartialEq, PartialOrd,
)]
// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering)]
pub struct DbObjectSnapshot {
    pub(super) definition: super::DbSchemaText,
    pub(super) name: super::DbSchemaText,
    pub(super) kind: super::DbObjectKind,
}

impl DbObjectSnapshot {
    #[must_use]
    pub const fn new(
        name: super::DbSchemaText,
        kind: super::DbObjectKind,
        definition: super::DbSchemaText,
    ) -> Self {
        Self {
            definition,
            name,
            kind,
        }
    }
}
