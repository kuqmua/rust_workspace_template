#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    proc_macro_getters::Getters,
    proc_macro_new::New,
)]
#[getters(get_mut)]
pub(crate) struct AdminAuthRequest {
    headers: crate::http_admin_header_map::HttpAdminHeaderMap,
    state: crate::shared_admin_auth_svc_state_arc::SharedAdminAuthSvcStateArc,
    peer: crate::admin_peer_addr::AdminPeerAddr,
}
