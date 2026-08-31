#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub(super) struct BoundedReadObservedBytes(usize);

impl BoundedReadObservedBytes {
    pub(crate) const fn get(self) -> usize {
        self.0
    }
}
