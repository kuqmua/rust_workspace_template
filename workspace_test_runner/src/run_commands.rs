pub(crate) fn run_commands(
    commands_ref: crate::commands_ref::CommandsRef<'_>,
) -> Result<(), crate::run_commands_error::RunCommandsError> {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let run_dir =
        std::path::Path::new(constants_str::WORKSPACE_TEST_RUNNER_RESULT_ROOT).join(format!(
            "{timestamp}-{}-{}",
            std::process::id(),
            crate::run_counter::RUN_COUNTER.fetch_add(1u64, std::sync::atomic::Ordering::Relaxed)
        ));
    std::fs::create_dir_all(run_dir.as_path()).map_err(|error| {
        crate::run_commands_error::RunCommandsError::CreateDirectory {
            execution_io_error: macro_helpers::std_tool_io_error::StdToolIoError::from(error),
        }
    })?;
    let mut command_runs = std::thread::scope(|scope| {
        commands_ref
            .iter()
            .enumerate()
            .map(|(index, (program, args))| {
                scope.spawn(move || {
                    let started_at =
                        crate::command_started_at_instant::CommandStartedAtInstant::from(
                            std::time::Instant::now(),
                        );
                    let output = macro_helpers::tool_command::ToolCommand::new(
                        macro_helpers::tool_program_ref::ToolProgramRef::from(*program),
                    )
                    .args(macro_helpers::tool_args_ref::ToolArgsRef::from(*args))
                    .output();
                    let command_index = crate::command_index::CommandIndex::from(index);
                    let mut command_failures_vec_deque = crate::command_failures_vec_deque::CommandFailuresVecDeque::default();
                    let (status_text, log_text) = match output {
                        Ok(command_output) => {
                            let stdout = String::from_utf8_lossy(command_output.stdout.as_slice());
                            let stderr = String::from_utf8_lossy(command_output.stderr.as_slice());
                            if !command_output.status.success() {
                                command_failures_vec_deque.push(crate::command_failure::CommandFailure::Exit {
                                    command_index,
                                    process_exit_status: macro_helpers::process_exit_status::ProcessExitStatus::from(command_output.status),
                                });
                            }
                            let mut write_output = |tool_console_stream: macro_helpers::tool_console_stream::ToolConsoleStream, std_fmt_arguments: macro_helpers::std_fmt_arguments::StdFmtArguments<'_>| {
                                if let Err(tool_console_write_error) = tool_console_stream.write(std_fmt_arguments) {
                                    command_failures_vec_deque.push(crate::command_failure::CommandFailure::WriteOutput {
                                        command_index,
                                        tool_console_write_error,
                                    });
                                }
                            };
                            write_output(macro_helpers::tool_console_stream::ToolConsoleStream::StandardOutput, macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!("{stdout}")));
                            write_output(macro_helpers::tool_console_stream::ToolConsoleStream::StandardError, macro_helpers::std_fmt_arguments::StdFmtArguments::from(format_args!("{stderr}")));
                            (command_output.status.to_string(), format!("{stdout}{stderr}"))
                        }
                        Err(error) => {
                            let status_text = format!("{}{}", constants_str::RUNNER_CLI_TEXT_5F9C0B1E, error);
                            let log_text = format!("{}{}{}", constants_str::RUNNER_CLI_TEXT_BE4BF145, error, constants_str::NEWLINE);
                            command_failures_vec_deque.push(crate::command_failure::CommandFailure::Spawn {
                                command_index,
                                execution_io_error: macro_helpers::std_tool_io_error::StdToolIoError::from(error),
                            });
                            (status_text, log_text)
                        }
                    };
                    crate::command_run::CommandRun::new(
                        command_failures_vec_deque,
                        crate::command_index::CommandIndex::from(index),
                        started_at.elapsed(),
                        crate::command_text::CommandText::try_from(log_text)
                            .unwrap_or_else(crate::command_text::CommandText::from),
                        crate::command_text::CommandText::try_from(status_text)
                            .unwrap_or_else(crate::command_text::CommandText::from),
                    )
                })
            })
            .collect::<Vec<_>>()
            .into_iter()
            .enumerate()
            .map(|(index, handle)| (crate::command_index::CommandIndex::from(index), handle.join()))
            .collect::<Vec<_>>()
    });
    let mut command_failures =
        crate::command_failures_vec_deque::CommandFailuresVecDeque::default();
    command_runs.sort_by_key(|(_, command_run_result)| match command_run_result {
        Ok(command_run) => usize::from(*command_run.get_command_index()),
        Err(_panic) => usize::MAX,
    });
    command_runs.iter_mut().for_each(
        |(command_index, command_run_result)| match command_run_result {
            Ok(command_run) => {
                command_failures.append(command_run.get_command_failures_vec_deque_mut());
            }
            Err(_panic) => {
                command_failures.push(crate::command_failure::CommandFailure::ThreadPanicked {
                    command_index: *command_index,
                });
            }
        },
    );
    let write_reports = || -> Result<(), crate::run_report_error::RunReportError> {
        let mut summary = crate::summary_text::SummaryText::default();
        command_runs.into_iter().try_for_each(|(command_index, command_run_result)| {
        let command_run = match command_run_result {
            Ok(command_run) => command_run,
            Err(_panic) => {
                summary.push_str(crate::text_ref::TextRef::from(
                    constants_str::COMMAND_THREAD_PANICKED_SUMMARY,
                ))?;
                return Ok(());
            }
        };
        let (program, args) = commands_ref
            .get(usize::from(*command_run.get_command_index()))
            .copied()
            .ok_or(crate::run_report_error::RunReportError::MissingCommand { command_index })?;
        let parts = std::iter::once(program)
            .chain(args.iter().copied())
            .take(3usize);
        let raw_capacity = parts
            .clone()
            .map(str::len)
            .sum::<usize>()
            .saturating_add(parts.clone().count().saturating_sub(constants_usize::ONE));
        let raw = parts.enumerate().fold(
            String::with_capacity(raw_capacity),
            |mut raw, (index, part)| {
                if index > constants_usize::ZERO {
                    raw.push_str(constants_str::HYPHEN);
                }
                raw.push_str(part);
                raw
            },
        );
        let sanitized = raw
            .chars()
            .map(|character| {
                if character.is_ascii_alphanumeric() {
                    character
                } else {
                    '-'
                }
            })
            .collect::<String>();
        let log_name = crate::command_text::CommandText::try_from(format!("{:02}{}{}{}", usize::from(*command_run.get_command_index()), constants_str::HYPHEN, sanitized, constants_str::RUNNER_CLI_TEXT_2E4BB066))
        .unwrap_or_else(crate::command_text::CommandText::from);
        let log_path = run_dir.join(log_name.as_ref());
        if let Err(error) = std::fs::write(log_path.as_path(), command_run.get_log_text().as_ref()) {
            return Err(crate::run_report_error::RunReportError::WriteLog {
                written_file_path_buf: macro_helpers::written_file_path_buf::WrittenFilePathBuf::from(log_path),
                execution_io_error: macro_helpers::std_tool_io_error::StdToolIoError::from(error),
            });
        }
        let failed_test_names = crate::failed_test_names::failed_test_names(crate::text_ref::TextRef::from(
            command_run.get_log_text().as_ref(),
        ));
        let failed_names_capacity = failed_test_names
            .as_ref()
            .iter()
            .map(|name| name.as_ref().len())
            .sum::<usize>()
            .saturating_add(
                failed_test_names
                    .as_ref()
                    .len()
                    .saturating_sub(constants_usize::ONE)
                    .saturating_mul(constants_str::TEXT_ALT_7.len()),
            );
        let failed_names = failed_test_names.as_ref().iter().enumerate().fold(
            String::with_capacity(failed_names_capacity),
            |mut names, (index, name)| {
                if index > constants_usize::ZERO {
                    names.push_str(constants_str::TEXT_ALT_7);
                }
                names.push_str(name.as_ref());
                names
            },
        );
        summary.push_str(
            crate::text_ref::TextRef::from(
                format!("{}{}{}{:?}{}{}{}{}{}{}{}{}{}", constants_str::RUNNER_CLI_TEXT_9C51337B, program, constants_str::RUNNER_CLI_TEXT_21B36EAB, args, constants_str::RUNNER_CLI_TEXT_169B9984, command_run.get_duration().as_millis(), constants_str::RUNNER_CLI_TEXT_1AF9787F, command_run.get_status_text().as_ref(), constants_str::RUNNER_CLI_TEXT_719096FE, log_path.display(), constants_str::RUNNER_CLI_TEXT_94C4B52B, failed_names, constants_str::NEWLINE)
            .as_str(),
            ),
        )?;
        Ok(())
    })?;
        std::fs::write(
            run_dir.join(constants_str::SUMMARY_TXT),
            crate::strip_ansi::strip_ansi(macro_helpers::tool_ansi_chars::ToolAnsiChars::from(
                macro_helpers::tool_ansi_text_ref::ToolAnsiTextRef::from(summary.as_ref()),
            ))
            .as_ref(),
        )
        .map_err(
            |error| crate::run_report_error::RunReportError::WriteSummary {
                execution_io_error: macro_helpers::std_tool_io_error::StdToolIoError::from(error),
            },
        )?;
        Ok(())
    };
    crate::run_commands_error::RunCommandsError::from_report_result(
        command_failures,
        write_reports(),
    )
}
