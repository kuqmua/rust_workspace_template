#[derive(Debug, thiserror::Error, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) enum RunCommandsError {
    #[error("{}", constants_str::RUNNER_COMMAND_FAILURE_MESSAGE)]
    CommandsFailed {
        command_failures: crate::command_failures_vec_deque::CommandFailuresVecDeque,
    },
    #[error("{message}: {execution_io_error}", message = constants_str::RUNNER_CREATE_DIRECTORY_MESSAGE)]
    CreateDirectory {
        #[source]
        execution_io_error: macro_helpers::std_tool_io_error::StdToolIoError,
    },
    #[error("{tool_console_write_error}")]
    WriteAnnouncement {
        #[source]
        tool_console_write_error: macro_helpers::tool_console_write_error::ToolConsoleWriteError,
    },
    #[error("{run_report_error}")]
    WriteReport {
        command_failures: crate::command_failures_vec_deque::CommandFailuresVecDeque,
        #[source]
        run_report_error: crate::run_report_error::RunReportError,
    },
}

impl RunCommandsError {
    #[allow(
        clippy::single_call_fn,
        reason = "the report result boundary is directly unit tested without filesystem or process access"
    )]
    pub(super) fn from_report_result(
        command_failures_vec_deque: crate::command_failures_vec_deque::CommandFailuresVecDeque,
        result: Result<(), crate::run_report_error::RunReportError>,
    ) -> Result<(), Self> {
        match result {
            Err(run_report_error) => Err(Self::WriteReport {
                command_failures: command_failures_vec_deque,
                run_report_error,
            }),
            Ok(()) if command_failures_vec_deque.is_empty() => Ok(()),
            Ok(()) => Err(Self::CommandsFailed {
                command_failures: command_failures_vec_deque,
            }),
        }
    }
}
