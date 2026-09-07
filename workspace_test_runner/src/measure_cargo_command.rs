pub(crate) fn measure_cargo_command(
    measurement_name: crate::measurement_name::MeasurementName,
    cargo_args: crate::cargo_args::CargoArgs,
) -> Result<(), crate::cargo_measurement_error::CargoMeasurementError> {
    let measurement_name_value = measurement_name.get();
    let started = std::time::Instant::now();
    let command_output = {
        let measurement_format = format!(
            "{}%M\n{}%R\n{}%F",
            constants_str::WORKSPACE_TEST_RUNNER_PEAK_RSS_PREFIX,
            constants_str::WORKSPACE_TEST_RUNNER_MINOR_PAGE_FAULTS_PREFIX,
            constants_str::WORKSPACE_TEST_RUNNER_MAJOR_PAGE_FAULTS_PREFIX,
        );
        macro_helpers::tool_command::ToolCommand::new(
            macro_helpers::tool_program_ref::ToolProgramRef::from(
                constants_str::WORKSPACE_TEST_RUNNER_TIME_PATH,
            ),
        )
        .arg(macro_helpers::tool_arg_ref::ToolArgRef::from(
            constants_str::F,
        ))
        .arg(macro_helpers::tool_arg_ref::ToolArgRef::from(
            measurement_format.as_str(),
        ))
        .arg(macro_helpers::tool_arg_ref::ToolArgRef::from(
            constants_str::WORKSPACE_TEST_RUNNER_CARGO,
        ))
        .args(macro_helpers::tool_args_ref::ToolArgsRef::from(
            cargo_args.get(),
        ))
        .output()
    };
    let duration = started.elapsed();
    match command_output {
        Ok(output) if output.status.success() => {
            let stderr = String::from_utf8_lossy(output.stderr.as_slice());
            let peak_rss_kb = stderr
                .lines()
                .find_map(|line| {
                    line.trim()
                        .strip_prefix(constants_str::WORKSPACE_TEST_RUNNER_PEAK_RSS_PREFIX)
                })
                .unwrap_or(constants_str::UNAVAILABLE);
            let minor_page_faults = stderr
                .lines()
                .find_map(|line| {
                    line.trim()
                        .strip_prefix(constants_str::WORKSPACE_TEST_RUNNER_MINOR_PAGE_FAULTS_PREFIX)
                })
                .unwrap_or(constants_str::UNAVAILABLE);
            let major_page_faults = stderr
                .lines()
                .find_map(|line| {
                    line.trim()
                        .strip_prefix(constants_str::WORKSPACE_TEST_RUNNER_MAJOR_PAGE_FAULTS_PREFIX)
                })
                .unwrap_or(constants_str::UNAVAILABLE);
            {
                let stdout = String::from_utf8_lossy(output.stdout.as_slice());
                if !stdout.is_empty() {
                    macro_helpers::tool_console_stream::ToolConsoleStream::StandardOutput
                        .write(macro_helpers::std_fmt_arguments::StdFmtArguments::from(
                            format_args!("{stdout}"),
                        ))
                        .map_err(|tool_console_write_error| {
                            crate::cargo_measurement_error::CargoMeasurementError::WriteOutput {
                                measurement_name,
                                tool_console_write_error,
                                process_exit_status:
                                    macro_helpers::process_exit_status::ProcessExitStatus::from(
                                        output.status,
                                    ),
                            }
                        })?;
                }
            }
            crate::print_without_measurement_footer::print_without_measurement_footer(
                crate::stderr_text_ref::StderrTextRef::from(stderr.as_ref()),
            )
            .map_err(|tool_console_write_error| {
                crate::cargo_measurement_error::CargoMeasurementError::WriteOutput {
                    measurement_name,
                    process_exit_status:
                        macro_helpers::process_exit_status::ProcessExitStatus::from(output.status),
                    tool_console_write_error,
                }
            })?;
            macro_helpers::tool_console_stream::ToolConsoleStream::StandardOutput
                .write(macro_helpers::std_fmt_arguments::StdFmtArguments::from(
                    format_args!(
                        "{}{}",
                        format_args!(
                            "{}{}{}{}{}{}{}{}{}{}{}",
                            constants_str::RUNNER_MEASUREMENT_PREFIX,
                            measurement_name_value,
                            constants_str::RUNNER_OUTPUT_WALL_MS,
                            duration.as_millis(),
                            constants_str::RUNNER_OUTPUT_MEMORY_PROXY_PEAK_RSS_KB,
                            peak_rss_kb,
                            constants_str::RUNNER_OUTPUT_MEMORY_PROXY_MINOR_PAGE_FAULTS,
                            minor_page_faults,
                            constants_str::RUNNER_OUTPUT_MEMORY_PROXY_MAJOR_PAGE_FAULTS,
                            major_page_faults,
                            constants_str::RUNNER_OUTPUT_STATUS_OK
                        ),
                        constants_str::NEWLINE
                    ),
                ))
                .map_err(|tool_console_write_error| {
                    crate::cargo_measurement_error::CargoMeasurementError::WriteOutput {
                        measurement_name,
                        tool_console_write_error,
                        process_exit_status:
                            macro_helpers::process_exit_status::ProcessExitStatus::from(
                                output.status,
                            ),
                    }
                })?;
            Ok(())
        }
        Ok(output) => {
            {
                let stdout = String::from_utf8_lossy(output.stdout.as_slice());
                if !stdout.is_empty() {
                    macro_helpers::tool_console_stream::ToolConsoleStream::StandardOutput
                        .write(macro_helpers::std_fmt_arguments::StdFmtArguments::from(
                            format_args!("{stdout}"),
                        ))
                        .map_err(|tool_console_write_error| {
                            crate::cargo_measurement_error::CargoMeasurementError::WriteOutput {
                                measurement_name,
                                tool_console_write_error,
                                process_exit_status:
                                    macro_helpers::process_exit_status::ProcessExitStatus::from(
                                        output.status,
                                    ),
                            }
                        })?;
                }
            }
            let stderr = String::from_utf8_lossy(output.stderr.as_slice());
            crate::print_without_measurement_footer::print_without_measurement_footer(
                crate::stderr_text_ref::StderrTextRef::from(stderr.as_ref()),
            )
            .map_err(|tool_console_write_error| {
                crate::cargo_measurement_error::CargoMeasurementError::WriteOutput {
                    measurement_name,
                    process_exit_status:
                        macro_helpers::process_exit_status::ProcessExitStatus::from(output.status),
                    tool_console_write_error,
                }
            })?;
            Err(
                crate::cargo_measurement_error::CargoMeasurementError::Exit {
                    measurement_name,
                    process_exit_status:
                        macro_helpers::process_exit_status::ProcessExitStatus::from(output.status),
                },
            )
        }
        Err(error) => Err(
            crate::cargo_measurement_error::CargoMeasurementError::Spawn {
                measurement_name,
                execution_io_error: macro_helpers::std_tool_io_error::StdToolIoError::from(error),
            },
        ),
    }
}
