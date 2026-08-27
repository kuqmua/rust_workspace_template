#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
pub(crate) struct ToolPath(&'static str);
impl ToolPath {
    pub(crate) const fn get(self) -> &'static str {
        self.0
    }
}
