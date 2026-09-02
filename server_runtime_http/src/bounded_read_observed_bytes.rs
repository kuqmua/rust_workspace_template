#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_newtype::FromInner,
)]
pub(super) struct BoundedReadObservedBytes(usize);

impl BoundedReadObservedBytes {
    pub(crate) const fn get(self) -> usize {
        self.0
    }
}
