pub trait SqlxPgPoolProvider {
    fn sqlx_pg_pool(&self) -> super::SqlxPgPoolRef<'_>;
}
