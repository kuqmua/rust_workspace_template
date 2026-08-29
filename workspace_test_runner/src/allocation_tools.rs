#[allow(clippy::single_call_fn)] // named command or composition stage has one orchestration owner
pub(crate) fn allocation_tools() -> [crate::allocation_tool::AllocationTool; 6] {
    [
        crate::allocation_tool::AllocationTool {
            name: crate::tool_name::ToolName::from(
                constants_str::catalog::WORKSPACE_TEST_RUNNER_LIBMEMUSAGE_TOOL,
            ),
            path: crate::tool_path::ToolPath::from(
                constants_str::catalog::WORKSPACE_TEST_RUNNER_MEMUSAGE_PATH,
            ),
        },
        crate::allocation_tool::AllocationTool {
            name: crate::tool_name::ToolName::from(
                constants_str::catalog::WORKSPACE_TEST_RUNNER_VALGRIND_TOOL,
            ),
            path: crate::tool_path::ToolPath::from(
                constants_str::catalog::WORKSPACE_TEST_RUNNER_VALGRIND_PATH,
            ),
        },
        crate::allocation_tool::AllocationTool {
            name: crate::tool_name::ToolName::from(
                constants_str::catalog::WORKSPACE_TEST_RUNNER_HEAPTRACK_TOOL,
            ),
            path: crate::tool_path::ToolPath::from(
                constants_str::catalog::WORKSPACE_TEST_RUNNER_HEAPTRACK_PATH,
            ),
        },
        crate::allocation_tool::AllocationTool {
            name: crate::tool_name::ToolName::from(
                constants_str::catalog::WORKSPACE_TEST_RUNNER_LTRACE_TOOL,
            ),
            path: crate::tool_path::ToolPath::from(
                constants_str::catalog::WORKSPACE_TEST_RUNNER_LTRACE_PATH,
            ),
        },
        crate::allocation_tool::AllocationTool {
            name: crate::tool_name::ToolName::from(
                constants_str::catalog::WORKSPACE_TEST_RUNNER_PERF_TOOL,
            ),
            path: crate::tool_path::ToolPath::from(
                constants_str::catalog::WORKSPACE_TEST_RUNNER_PERF_PATH,
            ),
        },
        crate::allocation_tool::AllocationTool {
            name: crate::tool_name::ToolName::from(constants_str::catalog::PG_CRUD_PG_TIME),
            path: crate::tool_path::ToolPath::from(
                constants_str::catalog::WORKSPACE_TEST_RUNNER_TIME_PATH,
            ),
        },
    ]
}
