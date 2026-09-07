#[derive(Debug, thiserror::Error, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub(super) enum CommandFailure {
    #[error("{message} {command_index}: {process_exit_status}", message = constants_str::RUNNER_COMMAND_FAILURE_MESSAGE)]
    Exit {
        command_index: crate::command_index::CommandIndex,
        process_exit_status: macro_helpers::process_exit_status::ProcessExitStatus,
    },
    #[error("{message} {command_index}: {execution_io_error}", message = constants_str::RUNNER_COMMAND_FAILURE_MESSAGE)]
    Spawn {
        command_index: crate::command_index::CommandIndex,
        #[source]
        execution_io_error: macro_helpers::std_tool_io_error::StdToolIoError,
    },
    #[error("{message} {command_index}", message = constants_str::RUNNER_THREAD_FAILURE_MESSAGE)]
    ThreadPanicked {
        command_index: crate::command_index::CommandIndex,
    },
    #[error("{message} {command_index}: {tool_console_write_error}", message = constants_str::RUNNER_COMMAND_FAILURE_MESSAGE)]
    WriteOutput {
        command_index: crate::command_index::CommandIndex,
        #[source]
        tool_console_write_error: macro_helpers::tool_console_write_error::ToolConsoleWriteError,
    },
}
