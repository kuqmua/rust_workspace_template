#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_getters::Getters,
)]
pub(crate) struct AdminPeerAddr(server_admin_core::admin_socket_addr::AdminSocketAddr);
impl AdminPeerAddr {
    pub(crate) const fn socket_addr(self) -> server_admin_core::admin_socket_addr::AdminSocketAddr {
        *self.get_inner()
    }
}
#[allow(
    unused_variables,
    reason = "extractor trait implementation preserves the repository type-based parameter name"
)]
impl<State> axum::extract::FromRequestParts<State> for AdminPeerAddr
where
    State: Send + Sync,
{
    type Rejection = crate::admin_error::AdminError;
    fn from_request_parts(
        parts: &mut http::request::Parts,
        state: &State,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> {
        std::future::ready(
            parts
                .extensions
                .get::<axum::extract::ConnectInfo<std::net::SocketAddr>>()
                .map(|value| {
                    Self::from(server_admin_core::admin_socket_addr::AdminSocketAddr::from(
                        value.0,
                    ))
                })
                .ok_or(crate::admin_error::AdminError::Authentication),
        )
    }
}
