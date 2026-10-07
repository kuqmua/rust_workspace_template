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

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_peer_extractor_rejects_missing_and_untyped_connection_information() {
        let (mut parts, ()) = http::Request::new(()).into_parts();
        let result = <crate::admin_peer_addr::AdminPeerAddr as axum::extract::FromRequestParts<
            (),
        >>::from_request_parts(&mut parts, &())
        .await;
        assert!(matches!(
            result,
            Err(crate::admin_error::AdminError::Authentication)
        ));
        let address = std::net::SocketAddr::from(([192u8, 0u8, 2u8, 10u8], 443u16));
        let previous = parts.extensions.insert(address);
        assert_eq!(previous, None);
        let untyped_connection_result = <crate::admin_peer_addr::AdminPeerAddr as axum::extract::FromRequestParts<
            (),
        >>::from_request_parts(&mut parts, &())
        .await;
        assert!(matches!(
            untyped_connection_result,
            Err(crate::admin_error::AdminError::Authentication)
        ));
        assert_eq!(
            parts.extensions.get::<std::net::SocketAddr>(),
            Some(&address)
        );
    }

    #[test]
    fn test_peer_extractor_preserves_ipv4_ipv6_ports_and_connection_extension() {
        [
            std::net::SocketAddr::from(([192u8, 0u8, 2u8, 10u8], 0u16)),
            std::net::SocketAddr::from(([192u8, 0u8, 2u8, 10u8], u16::MAX)),
            std::net::SocketAddr::from((
                [0x2001u16, 0xdb8u16, 0u16, 0u16, 0u16, 0u16, 0u16, 1u16],
                443u16,
            )),
        ].into_iter().fold((), |(), address| {
            let (mut parts, ()) = http::Request::new(()).into_parts();
            assert!(parts.extensions.insert(axum::extract::ConnectInfo(address)).is_none());
            let result = {
                let mut future = std::pin::pin!(<crate::admin_peer_addr::AdminPeerAddr as axum::extract::FromRequestParts<()>>::from_request_parts(&mut parts, &()));
                let mut context = std::task::Context::from_waker(std::task::Waker::noop());
                Future::poll(future.as_mut(), &mut context)
            };
            assert!(matches!(result, std::task::Poll::Ready(Ok(peer)) if peer.socket_addr() == server_admin_core::admin_socket_addr::AdminSocketAddr::from(address)));
            assert_eq!(parts.extensions.get::<axum::extract::ConnectInfo<std::net::SocketAddr>>().map(|connection| connection.0), Some(address));
        });
    }
}
