pub trait SqlxPgPoolProvider {
    fn sqlx_pg_pool(&self) -> crate::sqlx_pg_pool_ref::SqlxPgPoolRef<'_>;
}
