pub mod env_names {
    pub const CORS_ALLOW_ORIGIN: &str = "CORS_ALLOW_ORIGIN";
    pub const DATABASE_URL: &str = "DATABASE_URL";
    pub const ENABLE_API_GIT_COMMIT_CHECK: &str = "ENABLE_API_GIT_COMMIT_CHECK";
    pub const MAXIMUM_SIZE_OF_HTTP_BODY_IN_BYTES: &str = "MAXIMUM_SIZE_OF_HTTP_BODY_IN_BYTES";
    pub const PG_POOL_MAX_CONNECTIONS: &str = "PG_POOL_MAX_CONNECTIONS";
    pub const SERVICE_SOCKET_ADDRESS: &str = "SERVICE_SOCKET_ADDRESS";
    pub const SRC_PLACE_TYPE: &str = "SRC_PLACE_TYPE";
    pub const TIMEZONE: &str = "TIMEZONE";
    pub const TRACING_LEVEL: &str = "TRACING_LEVEL";
}
pub mod http_header_names {
    pub const X_API_GIT_COMMIT: &str = "x-api-git-commit";
    pub const X_REQUEST_ID: &str = "x-request-id";
}
pub mod route_paths {
    pub const HEALTH: &str = "/health";
    pub const NOT_FOUND: &str = "/404";
}
pub mod sql_names {
    pub const ID: &str = "id";
}
