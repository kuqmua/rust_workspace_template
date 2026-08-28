#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use super::SharedAdminAuthSvcStateArc;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner, newtype::IntoInnerFrom,
)]
pub(crate) struct AxumAdminStateRouter(pub(super) axum::Router<SharedAdminAuthSvcStateArc>);
