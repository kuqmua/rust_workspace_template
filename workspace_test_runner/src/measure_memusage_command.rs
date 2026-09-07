pub(super) fn measure_memusage_command(
    measurement_name: crate::measurement_name::MeasurementName,
    program_path_ref: crate::program_path_ref::ProgramPathRef<'_>,
    program_args_ref: crate::program_args_ref::ProgramArgsRef<'_>,
    memusage_prog_name_ref: crate::memusage_prog_name_ref::MemusageProgNameRef<'_>,
) -> Result<(), crate::memusage_measurement_error::MemusageMeasurementError> {
    let measurement_name_value = measurement_name.get();
    if !std::path::Path::new(constants_str::WORKSPACE_TEST_RUNNER_MEMUSAGE_PATH).exists() {
        macro_helpers::tool_console_stream::ToolConsoleStream::StandardOutput.write(macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!("{}{}", format_args!("{}{}{}{}", constants_str::RUNNER_MEASUREMENT_PREFIX, measurement_name_value, constants_str::RUNNER_OUTPUT_ALLOCATIONS_STATUS_UNAVAILABLE_REASON_LIBMEMUSAGE_NOT_FOUND_PATH, constants_str::WORKSPACE_TEST_RUNNER_MEMUSAGE_PATH), constants_str::NEWLINE))).map_err(|tool_console_write_error| crate::memusage_measurement_error::MemusageMeasurementError::WriteUnavailable {measurement_name, tool_console_write_error,})?;
        return Ok(());
    }
    let command_output = macro_helpers::tool_command::ToolCommand::new(
        macro_helpers::tool_program_ref::ToolProgramRef::from(program_path_ref.get()),
    )
    .args(macro_helpers::tool_args_ref::ToolArgsRef::from(
        program_args_ref.get(),
    ))
    .env(
        macro_helpers::tool_env_key_ref::ToolEnvKeyRef::from(constants_str::LD_PRELOAD),
        macro_helpers::tool_env_value_ref::ToolEnvValueRef::from(
            constants_str::WORKSPACE_TEST_RUNNER_MEMUSAGE_PATH,
        ),
    )
    .env(
        macro_helpers::tool_env_key_ref::ToolEnvKeyRef::from(constants_str::MEMUSAGE_PROG_NAME),
        macro_helpers::tool_env_value_ref::ToolEnvValueRef::from(memusage_prog_name_ref.get()),
    )
    .output();
    match command_output {
        Ok(output) if output.status.success() => {
            {
                let stdout = String::from_utf8_lossy(output.stdout.as_slice());
                if !stdout.is_empty() {
                    macro_helpers::tool_console_stream::ToolConsoleStream::StandardOutput.write(macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!("{stdout}"))).map_err(|tool_console_write_error| crate::memusage_measurement_error::MemusageMeasurementError::WriteOutput {measurement_name, tool_console_write_error,process_exit_status: macro_helpers::process_exit_status::ProcessExitStatus::from(output.status),})?;
                }
            }
            let stderr = String::from_utf8_lossy(output.stderr.as_slice());
            crate::print_without_memusage_footer::print_without_memusage_footer(
                crate::stderr_text_ref::StderrTextRef::from(stderr.as_ref()),
            )
            .map_err(|tool_console_write_error| {
                crate::memusage_measurement_error::MemusageMeasurementError::WriteOutput {
                    measurement_name,
                    process_exit_status:
                        macro_helpers::process_exit_status::ProcessExitStatus::from(output.status),
                    tool_console_write_error,
                }
            })?;
            let clean = crate::strip_ansi_codes::strip_ansi_codes(
                macro_helpers::tool_ansi_chars::ToolAnsiChars::from(
                    macro_helpers::tool_ansi_text_ref::ToolAnsiTextRef::from(stderr.as_ref()),
                ),
            );
            let heap_total = crate::memusage_heap_value::memusage_heap_value(
                &clean,
                crate::memusage_key::MemusageKey::from(constants_str::HEAP_TOTAL),
            )
            .get();
            let heap_peak = crate::memusage_heap_value::memusage_heap_value(
                &clean,
                crate::memusage_key::MemusageKey::from(constants_str::HEAP_PEAK),
            )
            .get();
            let stack_peak = crate::memusage_heap_value::memusage_heap_value(
                &clean,
                crate::memusage_key::MemusageKey::from(constants_str::STACK_PEAK),
            )
            .get();
            let malloc_calls = crate::memusage_table_value::memusage_table_value(
                &clean,
                crate::memusage_row_name::MemusageRowName::from(constants_str::MALLOC),
                crate::memory_usage_column_index::MemoryUsageColumnIndex::from(0),
            )
            .get();
            let malloc_memory = crate::memusage_table_value::memusage_table_value(
                &clean,
                crate::memusage_row_name::MemusageRowName::from(constants_str::MALLOC),
                crate::memory_usage_column_index::MemoryUsageColumnIndex::from(1),
            )
            .get();
            let malloc_failed = crate::memusage_table_value::memusage_table_value(
                &clean,
                crate::memusage_row_name::MemusageRowName::from(constants_str::MALLOC),
                crate::memory_usage_column_index::MemoryUsageColumnIndex::from(2),
            )
            .get();
            let realloc_calls = crate::memusage_table_value::memusage_table_value(
                &clean,
                crate::memusage_row_name::MemusageRowName::from(constants_str::REALLOC),
                crate::memory_usage_column_index::MemoryUsageColumnIndex::from(0),
            )
            .get();
            let realloc_memory = crate::memusage_table_value::memusage_table_value(
                &clean,
                crate::memusage_row_name::MemusageRowName::from(constants_str::REALLOC),
                crate::memory_usage_column_index::MemoryUsageColumnIndex::from(1),
            )
            .get();
            let realloc_failed = crate::memusage_table_value::memusage_table_value(
                &clean,
                crate::memusage_row_name::MemusageRowName::from(constants_str::REALLOC),
                crate::memory_usage_column_index::MemoryUsageColumnIndex::from(2),
            )
            .get();
            let calloc_calls = crate::memusage_table_value::memusage_table_value(
                &clean,
                crate::memusage_row_name::MemusageRowName::from(constants_str::CALLOC),
                crate::memory_usage_column_index::MemoryUsageColumnIndex::from(0),
            )
            .get();
            let calloc_memory = crate::memusage_table_value::memusage_table_value(
                &clean,
                crate::memusage_row_name::MemusageRowName::from(constants_str::CALLOC),
                crate::memory_usage_column_index::MemoryUsageColumnIndex::from(1),
            )
            .get();
            let calloc_failed = crate::memusage_table_value::memusage_table_value(
                &clean,
                crate::memusage_row_name::MemusageRowName::from(constants_str::CALLOC),
                crate::memory_usage_column_index::MemoryUsageColumnIndex::from(2),
            )
            .get();
            let free_calls = crate::memusage_table_value::memusage_table_value(
                &clean,
                crate::memusage_row_name::MemusageRowName::from(constants_str::FREE),
                crate::memory_usage_column_index::MemoryUsageColumnIndex::from(0),
            )
            .get();
            let free_memory = crate::memusage_table_value::memusage_table_value(
                &clean,
                crate::memusage_row_name::MemusageRowName::from(constants_str::FREE),
                crate::memory_usage_column_index::MemoryUsageColumnIndex::from(1),
            )
            .get();
            macro_helpers::tool_console_stream::ToolConsoleStream::StandardOutput.write(macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!("{}{}", format_args!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}", constants_str::RUNNER_MEASUREMENT_PREFIX, measurement_name_value, constants_str::RUNNER_OUTPUT_ALLOCATIONS_STATUS_OK_TOOL_LIBMEMUSAGE_HEAP_TOTAL_BYTES, heap_total, constants_str::RUNNER_OUTPUT_HEAP_PEAK_BYTES, heap_peak, constants_str::RUNNER_OUTPUT_STACK_PEAK_BYTES, stack_peak, constants_str::RUNNER_OUTPUT_MALLOC_CALLS, malloc_calls, constants_str::RUNNER_OUTPUT_MALLOC_BYTES, malloc_memory, constants_str::RUNNER_OUTPUT_MALLOC_FAILED, malloc_failed, constants_str::RUNNER_OUTPUT_REALLOC_CALLS, realloc_calls, constants_str::RUNNER_OUTPUT_REALLOC_BYTES, realloc_memory, constants_str::RUNNER_OUTPUT_REALLOC_FAILED, realloc_failed, constants_str::RUNNER_OUTPUT_CALLOC_CALLS, calloc_calls, constants_str::RUNNER_OUTPUT_CALLOC_BYTES, calloc_memory, constants_str::RUNNER_OUTPUT_CALLOC_FAILED, calloc_failed, constants_str::RUNNER_OUTPUT_FREE_CALLS, free_calls, constants_str::RUNNER_OUTPUT_FREE_BYTES, free_memory), constants_str::NEWLINE))).map_err(|tool_console_write_error| crate::memusage_measurement_error::MemusageMeasurementError::WriteOutput {measurement_name, tool_console_write_error,process_exit_status: macro_helpers::process_exit_status::ProcessExitStatus::from(output.status),})?;
            Ok(())
        }
        Ok(output) => {
            {
                let stdout = String::from_utf8_lossy(output.stdout.as_slice());
                if !stdout.is_empty() {
                    macro_helpers::tool_console_stream::ToolConsoleStream::StandardOutput.write(macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!("{stdout}"))).map_err(|tool_console_write_error| crate::memusage_measurement_error::MemusageMeasurementError::WriteOutput {measurement_name, tool_console_write_error,process_exit_status: macro_helpers::process_exit_status::ProcessExitStatus::from(output.status),})?;
                }
            }
            let stderr = String::from_utf8_lossy(output.stderr.as_slice());
            crate::print_without_memusage_footer::print_without_memusage_footer(
                crate::stderr_text_ref::StderrTextRef::from(stderr.as_ref()),
            )
            .map_err(|tool_console_write_error| {
                crate::memusage_measurement_error::MemusageMeasurementError::WriteOutput {
                    measurement_name,
                    process_exit_status:
                        macro_helpers::process_exit_status::ProcessExitStatus::from(output.status),
                    tool_console_write_error,
                }
            })?;
            Err(
                crate::memusage_measurement_error::MemusageMeasurementError::Exit {
                    measurement_name,
                    process_exit_status:
                        macro_helpers::process_exit_status::ProcessExitStatus::from(output.status),
                },
            )
        }
        Err(error) => Err(
            crate::memusage_measurement_error::MemusageMeasurementError::Spawn {
                measurement_name,
                execution_io_error: macro_helpers::std_tool_io_error::StdToolIoError::from(error),
            },
        ),
    }
}
