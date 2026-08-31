#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub struct BoundedReadConcurrencyMaximumNonZeroUsize(std::num::NonZeroUsize);

impl BoundedReadConcurrencyMaximumNonZeroUsize {
    pub(crate) const fn get(self) -> usize {
        self.0.get()
    }
}
