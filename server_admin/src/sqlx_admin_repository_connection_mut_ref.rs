#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    newtype::DerefInner,
    newtype::DerefMutInner,
    newtype::FromInner,
    generate_accessor::Getters,
)]
pub(crate) struct SqlxAdminRepositoryConnectionMutRef<'connection_lt>(
    &'connection_lt mut sqlx::PgConnection,
);
