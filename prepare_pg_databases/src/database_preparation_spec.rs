#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct DatabasePreparationSpec {
    migrations_source: crate::migrations_source::MigrationsSource,
    url: crate::database_url::DatabaseUrl,
}

impl DatabasePreparationSpec {
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        crate::database_url::DatabaseUrl,
        crate::migrations_source::MigrationsSource,
    ) {
        (self.url, self.migrations_source)
    }

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
