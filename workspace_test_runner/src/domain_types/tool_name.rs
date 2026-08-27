#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
pub(crate) struct ToolName(&'static str);
impl ToolName {
    pub(crate) const fn get(self) -> &'static str {
        self.0
    }
}
