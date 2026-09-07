#[derive(Debug, thiserror::Error, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) enum MemusageMeasurementError {
    #[error("{prefix}{measurement_name}{allocation} {status}{process_exit_status}", prefix = constants_str::RUNNER_MEASUREMENT_PREFIX, status = constants_str::RUNNER_MEASUREMENT_EXIT_PREFIX, allocation = constants_str::RUNNER_ALLOCATION_SUFFIX)]
    Exit {
        measurement_name: crate::measurement_name::MeasurementName,
        process_exit_status: macro_helpers::process_exit_status::ProcessExitStatus,
    },
    #[error("{prefix}{measurement_name}{allocation} {status}{execution_io_error}", prefix = constants_str::RUNNER_MEASUREMENT_PREFIX, status = constants_str::RUNNER_MEASUREMENT_SPAWN_PREFIX, allocation = constants_str::RUNNER_ALLOCATION_SUFFIX)]
    Spawn {
        measurement_name: crate::measurement_name::MeasurementName,
        #[source]
        execution_io_error: macro_helpers::std_tool_io_error::StdToolIoError,
    },
    #[error("{tool_console_write_error}")]
    WriteOutput {
        measurement_name: crate::measurement_name::MeasurementName,
        #[source]
        tool_console_write_error: macro_helpers::tool_console_write_error::ToolConsoleWriteError,
        process_exit_status: macro_helpers::process_exit_status::ProcessExitStatus,
    },
    #[error("{tool_console_write_error}")]
    WriteUnavailable {
        measurement_name: crate::measurement_name::MeasurementName,
        #[source]
        tool_console_write_error: macro_helpers::tool_console_write_error::ToolConsoleWriteError,
    },
}
