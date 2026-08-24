#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::AsRefInner,
    newtype::FromInner,
)]
pub struct SqlxPgPoolRef<'pool_lt>(&'pool_lt sqlx::PgPool);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    newtype::AsRefOwned,
    newtype::FromInner,
)]
pub struct SqlxPgPool(sqlx::PgPool);
pub trait GetSqlxPgPool {
    fn get_sqlx_pg_pool(&self) -> SqlxPgPoolRef<'_>;
}
