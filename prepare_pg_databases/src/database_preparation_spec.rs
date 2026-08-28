#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct DatabasePreparationSpec {
    pub(super) migrations_source: crate::domain_types::MigrationsSource,
    pub(super) url: crate::domain_types::DatabaseUrl,
}

impl DatabasePreparationSpec {
    #[must_use]
    pub const fn new(
        url: crate::domain_types::DatabaseUrl,
        migrations_source: crate::domain_types::MigrationsSource,
    ) -> Self {
        Self {
            migrations_source,
            url,
        }
    }
}
