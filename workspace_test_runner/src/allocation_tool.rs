#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
pub(crate) struct AllocationTool {
    pub(super) name: crate::tool_name::ToolName,
    pub(super) path: crate::tool_path::ToolPath,
}
impl AllocationTool {
    pub(crate) const fn name(self) -> crate::tool_name::ToolName {
        self.name
    }

    pub(crate) const fn path(self) -> crate::tool_path::ToolPath {
        self.path
    }
}
