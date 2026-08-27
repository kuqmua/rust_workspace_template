#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct DatabasePreparationSpec {
    pub(super) migrations_source: super::MigrationsSource,
    pub(super) url: super::DatabaseUrl,
}

impl DatabasePreparationSpec {
    #[must_use]
    pub const fn new(url: super::DatabaseUrl, migrations_source: super::MigrationsSource) -> Self {
        Self {
            migrations_source,
            url,
        }
    }
}
