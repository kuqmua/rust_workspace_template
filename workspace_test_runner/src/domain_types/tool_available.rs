#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
pub(crate) struct ToolAvailable(bool);
impl ToolAvailable {
    pub(crate) const fn get(self) -> bool {
        self.0
    }
}
