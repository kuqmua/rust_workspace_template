#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    generate_accessor::Getters,
    generate_constructor::New,
)]
pub(crate) struct AllocationTool {
    name: crate::tool_name::ToolName,
    path: crate::tool_path::ToolPath,
}
