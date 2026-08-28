#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::IntoInnerFrom, newtype::FromInner,
)]
pub struct AxumAdminResponse(pub(crate) axum::response::Response);
