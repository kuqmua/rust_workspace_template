#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner, newtype::IntoInnerFrom,
)]
pub(crate) struct AxumAdminStateRouter(
    pub(crate) axum::Router<crate::shared_admin_auth_svc_state_arc::SharedAdminAuthSvcStateArc>,
);
