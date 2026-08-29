#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use super::{
    AdminAuthPolicy, JsonwebtokenAdminDecodingKeys, JsonwebtokenAdminEncodingKey,
    StdAdminAccessTtlSeconds, StdAdminRefreshTtlSeconds, StdAdminSessionLimit,
};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub struct AdminAuthSvcState {
    pub(crate) access_ttl: StdAdminAccessTtlSeconds,
    pub(crate) allowed_origins: server_runtime_http::domain_types::AllowedOrigins,
    pub(crate) audience: config_lib::domain_types::AdminTokenAudience,
    pub(crate) decoding_keys: JsonwebtokenAdminDecodingKeys,
    pub(crate) encoding_key: JsonwebtokenAdminEncodingKey,
    pub(crate) issuer: config_lib::domain_types::AdminTokenIssuer,
    pub(crate) password_hasher: crate::AdminPasswordHasher,
    pub(crate) policy: AdminAuthPolicy,
    pub(crate) pool: app_state::SqlxPgPool,
    pub(crate) refresh_ttl: StdAdminRefreshTtlSeconds,
    pub(crate) session_limit: StdAdminSessionLimit,
    pub(crate) cookie_secure: crate::AdminCookieSecure,
}
