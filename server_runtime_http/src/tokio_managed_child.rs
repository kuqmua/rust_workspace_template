#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype_from_inner::FromInner,
)]
pub(super) struct TokioManagedChild(tokio::process::Child);

impl TokioManagedChild {
    pub(super) fn start_kill(&mut self) -> std::io::Result<()> {
        self.0.start_kill()
    }

    pub(super) async fn wait(&mut self) -> std::io::Result<std::process::ExitStatus> {
        self.0.wait().await
    }
}
