#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype_from_inner::FromInner,
)]
pub struct TokioOwnedSemaphorePermit(tokio::sync::OwnedSemaphorePermit);

impl TokioOwnedSemaphorePermit {
    pub fn forget(self) {
        self.0.forget();
    }
}
