pub use config_lib::{
    GetCorsAllowOrigin, GetDatabaseUrl, GetEnableApiGitCommitCheck,
    GetMaximumSizeOfHttpBodyInBytes, GetMongoUrl, GetPgPoolMaxConnections, GetRedisUrl,
    GetServiceSocketAddress, GetSrcPlaceType, GetStartingCheckLink, GetTimezone, GetTracingLevel,
    types::SrcPlaceType, types::TracingLevel,
};
#[derive(Debug, Clone, Copy)]
pub struct PgPoolRef<'pool_lt>(&'pool_lt sqlx::PgPool);
impl<'pool_lt> From<&'pool_lt sqlx::PgPool> for PgPoolRef<'pool_lt> {
    fn from(value: &'pool_lt sqlx::PgPool) -> Self {
        Self(value)
    }
}
impl AsRef<sqlx::PgPool> for PgPoolRef<'_> {
    fn as_ref(&self) -> &sqlx::PgPool {
        self.0
    }
}
#[derive(Debug, Clone)]
pub struct PgPool(sqlx::PgPool);
impl From<sqlx::PgPool> for PgPool {
    fn from(value: sqlx::PgPool) -> Self {
        Self(value)
    }
}
impl AsRef<sqlx::PgPool> for PgPool {
    fn as_ref(&self) -> &sqlx::PgPool {
        &self.0
    }
}
pub trait GetPgPool {
    fn get_pg_pool(&self) -> PgPoolRef<'_>;
}
