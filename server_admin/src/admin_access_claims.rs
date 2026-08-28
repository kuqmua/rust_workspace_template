#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use super::*;
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
    pub(super) aud: config_lib::domain_types::AdminTokenAudience,
    pub(super) exp: AdminUnixTokenStream,
    pub(super) iat: AdminUnixTokenStream,
    pub(super) iss: config_lib::domain_types::AdminTokenIssuer,
    pub(super) sub: super::super::AdminUserId,
    pub(super) jti: AdminSessionId,
}
