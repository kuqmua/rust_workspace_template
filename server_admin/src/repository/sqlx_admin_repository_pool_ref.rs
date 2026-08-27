#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::DerefInner,
    newtype::FromInner,
)]
pub(crate) struct SqlxAdminRepositoryPoolRef<'pool_lt>(pub(super) &'pool_lt sqlx::PgPool);
