#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub struct TokioTcpListener(pub(super) tokio::net::TcpListener);
