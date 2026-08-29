#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub struct AdminAuthSvcState {
    pub(crate) access_ttl: crate::std_admin_access_ttl_seconds::StdAdminAccessTtlSeconds,
    pub(crate) allowed_origins: server_runtime_http::allowed_origins::AllowedOrigins,
    pub(crate) audience: config_lib::admin_token_audience::AdminTokenAudience,
    pub(crate) decoding_keys:
        crate::jsonwebtoken_admin_decoding_keys::JsonwebtokenAdminDecodingKeys,
    pub(crate) encoding_key: crate::jsonwebtoken_admin_encoding_key::JsonwebtokenAdminEncodingKey,
    pub(crate) issuer: config_lib::admin_token_issuer::AdminTokenIssuer,
    pub(crate) password_hasher: crate::admin_password_hasher::AdminPasswordHasher,
    pub(crate) policy: crate::admin_auth_policy::AdminAuthPolicy,
    pub(crate) pool: app_state::sqlx_pg_pool::SqlxPgPool,
    pub(crate) refresh_ttl: crate::std_admin_refresh_ttl_seconds::StdAdminRefreshTtlSeconds,
    pub(crate) session_limit: crate::std_admin_session_limit::StdAdminSessionLimit,
    pub(crate) cookie_secure: crate::admin_cookie_secure::AdminCookieSecure,
}
