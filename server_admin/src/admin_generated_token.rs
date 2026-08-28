#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use crate::*;
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub struct AdminGeneratedToken {
    pub(crate) hash: AdminTokenHash,
    pub(crate) token: AdminOpaqueToken,
}
