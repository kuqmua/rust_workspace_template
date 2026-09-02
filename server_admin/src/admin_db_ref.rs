#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) enum AdminDbRef<'connection_lt, 'pool_lt> {
    Connection(
        crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef<
            'connection_lt,
        >,
    ),
    Pool(crate::sqlx_admin_repository_pool_ref::SqlxAdminRepositoryPoolRef<'pool_lt>),
}
