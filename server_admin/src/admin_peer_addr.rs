#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::FromInner,
    generate_accessor::Getters,
)]
pub(crate) struct AdminPeerAddr(server_admin_core::admin_socket_addr::AdminSocketAddr);
impl AdminPeerAddr {
    pub(crate) const fn socket_addr(self) -> server_admin_core::admin_socket_addr::AdminSocketAddr {
        *self.get_inner()
    }
}
