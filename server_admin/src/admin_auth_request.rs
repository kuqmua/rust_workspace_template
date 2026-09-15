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
impl
    axum::extract::FromRequestParts<
        crate::shared_admin_auth_svc_state_arc::SharedAdminAuthSvcStateArc,
    > for AdminAuthRequest
{
    type Rejection = crate::admin_error::AdminError;
    fn from_request_parts(
        parts: &mut http::request::Parts,
        shared_admin_auth_svc_state_arc: &crate::shared_admin_auth_svc_state_arc::SharedAdminAuthSvcStateArc,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> {
        std::future::ready(
            parts
                .extensions
                .get::<axum::extract::ConnectInfo<std::net::SocketAddr>>()
                .map(|peer| {
                    Self::new(
                        crate::http_admin_header_map::HttpAdminHeaderMap::from(
                            parts.headers.clone(),
                        ),
                        shared_admin_auth_svc_state_arc.clone(),
                        crate::admin_peer_addr::AdminPeerAddr::from(
                            server_admin_core::admin_socket_addr::AdminSocketAddr::from(peer.0),
                        ),
                    )
                })
                .ok_or(crate::admin_error::AdminError::Authentication),
        )
    }
}
