pub use config_lib::{
    GetCorsAllowOrigin, GetDatabaseUrl, GetEnableApiGitCommitCheck,
    GetMaximumSizeOfHttpBodyInBytes, GetMongoUrl, GetPgPoolMaxConnections, GetRedisUrl,
    GetServiceSocketAddress, GetSrcPlaceType, GetStartingCheckLink, GetTimezone, GetTracingLevel,
    types::{SrcPlaceType, TracingLevel},
};
use sqlx::PgPool;
pub trait GetPgPool {
    fn get_pg_pool(&self) -> &PgPool;
}
