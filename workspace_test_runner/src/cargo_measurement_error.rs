#[derive(Debug, thiserror::Error, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) enum CargoMeasurementError {
    #[error("{prefix}{measurement_name} {status}{process_exit_status}", prefix = constants_str::RUNNER_MEASUREMENT_PREFIX, status = constants_str::RUNNER_MEASUREMENT_EXIT_PREFIX)]
    Exit {
        measurement_name: crate::measurement_name::MeasurementName,
        process_exit_status: macro_helpers::process_exit_status::ProcessExitStatus,
    },
    #[error("{prefix}{measurement_name} {status}{execution_io_error}", prefix = constants_str::RUNNER_MEASUREMENT_PREFIX, status = constants_str::RUNNER_MEASUREMENT_SPAWN_PREFIX)]
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
}
