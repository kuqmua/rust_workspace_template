#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_newtype::FromInner,
)]
pub struct HttpAcceptHeaderMaximumBytes(usize);

impl HttpAcceptHeaderMaximumBytes {
    pub(crate) const fn get(self) -> usize {
        self.0
    }
}
