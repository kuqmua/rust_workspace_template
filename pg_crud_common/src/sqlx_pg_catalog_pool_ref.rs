#[derive(
    proc_macro_getters::Getters,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_newtype::FromInner,
)]
pub struct SqlxPgCatalogPoolRef<'value_lt>(&'value_lt sqlx::PgPool);
