#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    newtype::DebugTransparent,
    newtype::FromInner,
)]
pub struct Argon2AdminPasswordHashError(pub(super) argon2::password_hash::Error);
