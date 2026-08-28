use super::{AllocationTool, ToolName, ToolPath};

#[allow(clippy::single_call_fn)] // runtime construction keeps wrapper initialization on From while centralizing tool metadata
pub(crate) fn allocation_tools() -> [AllocationTool; 6] {
    [
        AllocationTool {
            name: ToolName::from(constants_str::WORKSPACE_TEST_RUNNER_LIBMEMUSAGE_TOOL),
            path: ToolPath::from(constants_str::WORKSPACE_TEST_RUNNER_MEMUSAGE_PATH),
        },
        AllocationTool {
            name: ToolName::from(constants_str::WORKSPACE_TEST_RUNNER_VALGRIND_TOOL),
            path: ToolPath::from(constants_str::WORKSPACE_TEST_RUNNER_VALGRIND_PATH),
        },
        AllocationTool {
            name: ToolName::from(constants_str::WORKSPACE_TEST_RUNNER_HEAPTRACK_TOOL),
            path: ToolPath::from(constants_str::WORKSPACE_TEST_RUNNER_HEAPTRACK_PATH),
        },
        AllocationTool {
            name: ToolName::from(constants_str::WORKSPACE_TEST_RUNNER_LTRACE_TOOL),
            path: ToolPath::from(constants_str::WORKSPACE_TEST_RUNNER_LTRACE_PATH),
        },
        AllocationTool {
            name: ToolName::from(constants_str::WORKSPACE_TEST_RUNNER_PERF_TOOL),
            path: ToolPath::from(constants_str::WORKSPACE_TEST_RUNNER_PERF_PATH),
        },
        AllocationTool {
            name: ToolName::from(constants_str::PG_CRUD_PG_TIME),
            path: ToolPath::from(constants_str::WORKSPACE_TEST_RUNNER_TIME_PATH),
        },
    ]
}
