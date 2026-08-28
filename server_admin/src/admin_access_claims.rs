#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use crate::*;
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
    pub(crate) aud: config_lib::domain_types::AdminTokenAudience,
    pub(crate) exp: AdminUnixTokenStream,
    pub(crate) iat: AdminUnixTokenStream,
    pub(crate) iss: config_lib::domain_types::AdminTokenIssuer,
    pub(crate) sub: AdminUserId,
    pub(crate) jti: AdminSessionId,
}
