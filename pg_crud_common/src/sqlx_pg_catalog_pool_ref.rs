#[derive(
    generate_accessor::Getters,
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::FromInner,
)]
pub struct SqlxPgCatalogPoolRef<'value_lt>(&'value_lt sqlx::PgPool);
