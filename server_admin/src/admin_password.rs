#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::DebugRedacted,
    newtype::FromInner,
    serde::Deserialize,
)]
#[serde(try_from = "String")]
pub struct AdminPassword(pub(super) super::super::SecrecyAdminString);
