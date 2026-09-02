#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    proc_macro_newtype::AsRefOwned,
    proc_macro_newtype::FromInner,
)]
pub struct SqlxPgPool(sqlx::PgPool);
