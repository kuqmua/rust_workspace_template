#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PgPool;

pub trait GetCorsAllowOrigin {
    fn get_cors_allow_origin(&self) -> &config_lib::CorsAllowOrigin;
}

pub trait GetDatabaseUrl {
    fn get_database_url(&self) -> &config_lib::DatabaseUrl;
}

pub trait GetEnableApiGitCommitCheck {
    fn get_enable_api_git_commit_check(&self) -> &config_lib::EnableApiGitCommitCheck;
}

pub trait GetMaximumSizeOfHttpBodyInBytes {
    fn get_maximum_size_of_http_body_in_bytes(&self) -> &config_lib::MaximumSizeOfHttpBodyInBytes;
}

pub trait GetMongoUrl {
    fn get_mongo_url(&self) -> &config_lib::MongoUrl;
}

pub trait GetPgPool {
    fn get_pg_pool(&self) -> &PgPool;
}

pub trait GetPgPoolMaxConnections {
    fn get_pg_pool_max_connections(&self) -> &config_lib::PgPoolMaxConnections;
}

pub trait GetRedisUrl {
    fn get_redis_url(&self) -> &config_lib::RedisUrl;
}

pub trait GetServiceSocketAddress {
    fn get_service_socket_address(&self) -> &config_lib::ServiceSocketAddress;
}

pub trait GetSrcPlaceType {
    fn get_src_place_type(&self) -> &config_lib::SrcPlaceType;
}

pub trait GetStartingCheckLink {
    fn get_starting_check_link(&self) -> &config_lib::StartingCheckLink;
}

pub trait GetTimezone {
    fn get_timezone(&self) -> &config_lib::Timezone;
}

pub trait GetTracingLevel {
    fn get_tracing_level(&self) -> &config_lib::TracingLevel;
}
