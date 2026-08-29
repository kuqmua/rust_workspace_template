#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone)]
pub(crate) struct AdminAuthReq {
    pub(crate) headers: crate::http_admin_header_map::HttpAdminHeaderMap,
    pub(crate) state: crate::shared_admin_auth_svc_state_arc::SharedAdminAuthSvcStateArc,
    pub(crate) peer: crate::admin_peer_addr::AdminPeerAddr,
}
