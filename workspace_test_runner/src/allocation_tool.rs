#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    proc_macro_getters::Getters,
    proc_macro_new::New,
)]
pub(crate) struct AllocationTool {
    name: crate::tool_name::ToolName,
    path: crate::tool_path::ToolPath,
}
