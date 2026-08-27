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
    pub(super) access_ttl: StdAdminAccessTtlSeconds,
    pub(super) allowed_origins: server_runtime_http::domain_types::AllowedOrigins,
    pub(super) audience: config_lib::domain_types::AdminTokenAudience,
    pub(super) decoding_keys: JsonwebtokenAdminDecodingKeys,
    pub(super) encoding_key: JsonwebtokenAdminEncodingKey,
    pub(super) issuer: config_lib::domain_types::AdminTokenIssuer,
    pub(super) password_hasher: super::super::AdminPasswordHasher,
    pub(super) policy: AdminAuthPolicy,
    pub(super) pool: app_state::domain_types::SqlxPgPool,
    pub(super) refresh_ttl: StdAdminRefreshTtlSeconds,
    pub(super) session_limit: StdAdminSessionLimit,
    pub(super) cookie_secure: super::super::AdminCookieSecure,
}
