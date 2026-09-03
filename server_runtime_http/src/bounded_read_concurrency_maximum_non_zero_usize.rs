#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_newtype_from_inner::FromInner,
)]
pub struct BoundedReadConcurrencyMaximumNonZeroUsize(std::num::NonZeroUsize);

impl BoundedReadConcurrencyMaximumNonZeroUsize {
    pub(crate) const fn get(self) -> usize {
        self.0.get()
    }
}
