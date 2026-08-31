#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::Display,
    newtype::FromInner,
)]
pub struct ClientSocketAddr(std::net::SocketAddr);

impl ClientSocketAddr {
    pub(crate) const fn ip(self) -> std::net::IpAddr {
        self.0.ip()
    }
}
