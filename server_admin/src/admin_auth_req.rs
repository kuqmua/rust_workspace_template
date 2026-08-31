#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    generate_accessor::Getters,
    generate_constructor::New,
)]
#[getters(get_mut)]
pub(crate) struct AdminAuthReq {
    headers: crate::http_admin_header_map::HttpAdminHeaderMap,
    state: crate::shared_admin_auth_svc_state_arc::SharedAdminAuthSvcStateArc,
    peer: crate::admin_peer_addr::AdminPeerAddr,
}
