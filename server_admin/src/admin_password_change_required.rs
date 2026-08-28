#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    serde::Serialize,
    newtype::DerefInner,
    newtype::FromInner,
)]
#[serde(transparent)]
pub(crate) struct AdminPasswordChangeRequired(pub(crate) bool);
