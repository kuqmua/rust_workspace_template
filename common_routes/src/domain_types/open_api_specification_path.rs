#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(
    Debug,
    Clone,
    Copy,
    serde::Serialize,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::FromInner,
)]
pub(super) struct OpenApiSpecificationPath(pub(super) &'static str);
