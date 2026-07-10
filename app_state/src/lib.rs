#[derive(Debug, Clone, Copy, newtype::Newtype)]
#[newtype(as_ref_inner, from_inner)]
pub struct SqlxPgPoolRef<'pool_lt>(&'pool_lt sqlx::PgPool);
#[derive(Debug, Clone, newtype::Newtype)]
#[newtype(as_ref_owned, from_inner)]
pub struct SqlxPgPool(sqlx::PgPool);
pub trait GetSqlxPgPool {
    fn get_sqlx_pg_pool(&self) -> SqlxPgPoolRef<'_>;
}
