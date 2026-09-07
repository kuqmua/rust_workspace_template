#[derive(Debug, thiserror::Error, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub(super) enum RunReportError {
    #[error(transparent)]
    AppendSummary(#[from] crate::summary_text_append_error::SummaryTextAppendError),
    #[error("{message} {command_index}", message = constants_str::RUNNER_COMMAND_MISSING_MESSAGE)]
    MissingCommand {
        command_index: crate::command_index::CommandIndex,
    },
    #[error("{message} {}: {execution_io_error}", written_file_path_buf.as_ref().display(), message = constants_str::RUNNER_WRITE_LOG_MESSAGE)]
    WriteLog {
        written_file_path_buf: macro_helpers::written_file_path_buf::WrittenFilePathBuf,
        #[source]
        execution_io_error: macro_helpers::std_tool_io_error::StdToolIoError,
    },
    #[error("{message}: {execution_io_error}", message = constants_str::RUNNER_WRITE_SUMMARY_MESSAGE)]
    WriteSummary {
        #[source]
        execution_io_error: macro_helpers::std_tool_io_error::StdToolIoError,
    },
}
