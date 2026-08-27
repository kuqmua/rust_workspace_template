#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    PartialEq,
    newtype::FromInner,
    newtype::GetInner,
    newtype::IntoInnerFrom,
    newtype::Display,
)]
pub struct StdStaleStagingEntryCount(pub(super) usize);
