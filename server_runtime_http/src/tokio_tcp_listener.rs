#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub struct TokioTcpListener(tokio::net::TcpListener);

impl TokioTcpListener {
    pub(crate) fn into_inner(self) -> tokio::net::TcpListener {
        self.0
    }
}
