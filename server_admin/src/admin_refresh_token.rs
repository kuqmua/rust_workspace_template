#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use crate::*;
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, newtype::DebugRedacted, newtype::FromInner,
)]
pub struct AdminRefreshToken(pub(crate) AdminOpaqueToken);
