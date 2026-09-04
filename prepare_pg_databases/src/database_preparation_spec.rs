#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_new::New,
)]
pub struct DatabasePreparationSpec {
    #[constructor(order = 1)]
    migrations_source: crate::migrations_source::MigrationsSource,
    #[constructor(order = 0)]
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
}
