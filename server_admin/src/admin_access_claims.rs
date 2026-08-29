#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct AdminAccessClaims {
    pub(crate) aud: config_lib::admin_token_audience::AdminTokenAudience,
    pub(crate) exp: crate::admin_unix_token_stream::AdminUnixTokenStream,
    pub(crate) iat: crate::admin_unix_token_stream::AdminUnixTokenStream,
    pub(crate) iss: config_lib::admin_token_issuer::AdminTokenIssuer,
    pub(crate) sub: server_admin_core::admin_user_id::AdminUserId,
    pub(crate) jti: crate::admin_session_id::AdminSessionId,
}
