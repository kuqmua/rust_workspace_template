#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    newtype::DerefInner,
    newtype::DerefMutInner,
    newtype::FromInner,
)]
pub(crate) struct SqlxAdminRepositoryConnectionMutRef<'connection_lt>(
    pub(super) &'connection_lt mut sqlx::PgConnection,
);
