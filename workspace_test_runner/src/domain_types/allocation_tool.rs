use super::{ToolName, ToolPath};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
pub(crate) struct AllocationTool {
    pub(super) name: ToolName,
    pub(super) path: ToolPath,
}
impl AllocationTool {
    pub(crate) const fn name(self) -> ToolName {
        self.name
    }

    pub(crate) const fn path(self) -> ToolPath {
        self.path
    }
}
