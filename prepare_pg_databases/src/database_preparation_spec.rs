#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct DatabasePreparationSpec {
    pub(super) migrations_source: crate::migrations_source::MigrationsSource,
    pub(super) url: crate::database_url::DatabaseUrl,
}

impl DatabasePreparationSpec {
    #[must_use]
    pub const fn new(
        url: crate::database_url::DatabaseUrl,
        migrations_source: crate::migrations_source::MigrationsSource,
    ) -> Self {
        Self {
            migrations_source,
            url,
        }
    }
}
