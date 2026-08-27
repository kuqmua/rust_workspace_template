#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    newtype::AsRefOwned,
    newtype::FromInner,
)]
pub struct SqlxPgPool(sqlx::PgPool);
