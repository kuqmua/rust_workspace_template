#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, proc_macro_newtype::FromInner,
)]
pub struct TokioTcpListener(tokio::net::TcpListener);

impl TokioTcpListener {
    pub(crate) fn into_inner(self) -> tokio::net::TcpListener {
        self.0
    }
}
