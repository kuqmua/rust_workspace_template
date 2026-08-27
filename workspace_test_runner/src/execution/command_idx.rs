#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub(super) struct CommandIdx(pub(super) usize);
impl CommandIdx {
    pub(super) const fn get(self) -> usize {
        self.0
    }
}
