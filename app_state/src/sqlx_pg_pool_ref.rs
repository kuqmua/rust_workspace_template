#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::AsRefInner,
    newtype::FromInner,
)]
pub struct SqlxPgPoolRef<'pool_lt>(&'pool_lt sqlx::PgPool);
