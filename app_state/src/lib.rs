pub use config_lib::{
    GetCorsAllowOrigin, GetDatabaseUrl, GetEnableApiGitCommitCheck,
    GetMaximumSizeOfHttpBodyInBytes, GetMongoUrl, GetPgPoolMaxConnections, GetRedisUrl,
    GetServiceSocketAddress, GetSrcPlaceType, GetStartingCheckLink, GetTimezone, GetTracingLevel,
    types::SrcPlaceType, types::TracingLevel,
};
#[derive(Debug, Clone, Copy)]
pub struct PgPoolRef<'pool_lt>(pub &'pool_lt sqlx::PgPool);
#[derive(Debug, Clone)]
pub struct PgPool(pub sqlx::PgPool);
pub trait GetPgPool {
    fn get_pg_pool(&self) -> PgPoolRef<'_>;
}
