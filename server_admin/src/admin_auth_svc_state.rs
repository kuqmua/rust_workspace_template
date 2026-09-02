#[derive(
    proc_macro_new::New,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_getters::Getters,
)]
pub struct AdminAuthSvcState {
    access_ttl: crate::std_admin_access_ttl_seconds::StdAdminAccessTtlSeconds,
    allowed_origins: server_runtime_http::allowed_origins::AllowedOrigins,
    audience: config_lib::admin_token_audience::AdminTokenAudience,
    decoding_keys: crate::jsonwebtoken_admin_decoding_keys::JsonwebtokenAdminDecodingKeys,
    encoding_key: crate::jsonwebtoken_admin_encoding_key::JsonwebtokenAdminEncodingKey,
    issuer: config_lib::admin_token_issuer::AdminTokenIssuer,
    password_hasher: crate::admin_password_hasher::AdminPasswordHasher,
    policy: crate::admin_auth_policy::AdminAuthPolicy,
    pool: app_state::sqlx_pg_pool::SqlxPgPool,
    refresh_ttl: crate::std_admin_refresh_ttl_seconds::StdAdminRefreshTtlSeconds,
    session_limit: crate::std_admin_session_limit::StdAdminSessionLimit,
    cookie_secure: crate::runtime_admin_cookie_secure::RuntimeAdminCookieSecure,
}
