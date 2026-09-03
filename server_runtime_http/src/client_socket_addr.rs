#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_display::Display,
    proc_macro_newtype_from_inner::FromInner,
)]
pub struct ClientSocketAddr(std::net::SocketAddr);

impl ClientSocketAddr {
    pub(crate) const fn ip(self) -> std::net::IpAddr {
        self.0.ip()
    }
}
