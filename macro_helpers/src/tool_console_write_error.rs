#[derive(Debug, thiserror::Error, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub enum ToolConsoleWriteError {
    #[error("{message}: {0}", message = constants_str::TOOL_STDERR_WRITE_FAILED)]
    StandardError(#[source] crate::std_tool_io_error::StdToolIoError),
    #[error("{message}: {0}", message = constants_str::TOOL_STDOUT_WRITE_FAILED)]
    StandardOutput(#[source] crate::std_tool_io_error::StdToolIoError),
}
