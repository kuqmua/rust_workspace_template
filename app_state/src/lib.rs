#[derive(Debug, Clone, Copy)]
pub struct SqlxPgPoolRef<'pool_lt>(&'pool_lt sqlx::PgPool);
impl<'pool_lt> From<&'pool_lt sqlx::PgPool> for SqlxPgPoolRef<'pool_lt> {
    fn from(value: &'pool_lt sqlx::PgPool) -> Self {
        Self(value)
    }
}
impl AsRef<sqlx::PgPool> for SqlxPgPoolRef<'_> {
    fn as_ref(&self) -> &sqlx::PgPool {
        self.0
    }
}
#[derive(Debug, Clone)]
pub struct SqlxPgPool(sqlx::PgPool);
impl From<sqlx::PgPool> for SqlxPgPool {
    fn from(value: sqlx::PgPool) -> Self {
        Self(value)
    }
}
impl AsRef<sqlx::PgPool> for SqlxPgPool {
    fn as_ref(&self) -> &sqlx::PgPool {
        &self.0
    }
}
pub trait GetSqlxPgPool {
    fn get_sqlx_pg_pool(&self) -> SqlxPgPoolRef<'_>;
}
