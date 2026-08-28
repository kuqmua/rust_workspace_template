#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) enum AdminDbRef<'connection_lt, 'pool_lt> {
    Connection(crate::repository::SqlxAdminRepositoryConnectionMutRef<'connection_lt>),
    Pool(crate::repository::SqlxAdminRepositoryPoolRef<'pool_lt>),
}
