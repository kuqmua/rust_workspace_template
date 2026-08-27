#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
pub(crate) struct MemusageColumnIdx(usize);
impl MemusageColumnIdx {
    pub(crate) const fn get(self) -> usize {
        self.0
    }
}
