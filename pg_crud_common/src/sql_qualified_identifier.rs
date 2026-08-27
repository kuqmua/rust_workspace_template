#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "split owner modules expose representation only within the crate"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct SqlQualifiedIdentifier {
    pub(crate) schema: crate::domain_types::SqlIdentifier,
    pub(crate) table: crate::domain_types::SqlIdentifier,
}

impl SqlQualifiedIdentifier {
    #[must_use]
    pub const fn new(
        schema: crate::domain_types::SqlIdentifier,
        table: crate::domain_types::SqlIdentifier,
    ) -> Self {
        Self { schema, table }
    }
}

impl std::fmt::Display for SqlQualifiedIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.schema.as_ref())?;
        f.write_str(constants_str::DOT)?;
        f.write_str(self.table.as_ref())
    }
}
