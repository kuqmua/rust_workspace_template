#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::DerefInner,
    newtype::FromInner,
    generate_accessor::Getters,
)]
pub(crate) struct SqlxAdminRepositoryPoolRef<'pool_lt>(&'pool_lt sqlx::PgPool);
