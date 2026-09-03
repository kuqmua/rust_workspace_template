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
