#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub struct AdminGeneratedToken {
    pub(crate) hash: crate::admin_token_hash::AdminTokenHash,
    pub(crate) token: crate::admin_opaque_token::AdminOpaqueToken,
}
