#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(
    Debug,
    Clone,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::IntoInnerFrom,
    newtype::FromInner,
)]
pub struct AxumCommonRoutes(pub(super) axum::Router);
