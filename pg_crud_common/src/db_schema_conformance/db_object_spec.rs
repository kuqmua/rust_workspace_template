#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
#[allow(clippy::arbitrary_source_item_ordering)]
pub struct DbObjectSpec {
    pub(super) definition: super::DbStaticSchemaText,
    pub(super) name: super::DbStaticSchemaText,
    pub(super) kind: super::DbObjectKind,
}

impl DbObjectSpec {
    #[must_use]
    pub const fn new(
        name: super::DbStaticSchemaText,
        kind: super::DbObjectKind,
        definition: super::DbStaticSchemaText,
    ) -> Self {
        Self {
            definition,
            name,
            kind,
        }
    }
}
