use crate::execution::{
    CommandIdx, CommandRun, CommandStartedAtInstant, CommandSucceeded, CommandText, CommandsRef,
    ExecutionIoError, RUN_COUNTER, SummaryText, TextRef, failed_test_names, strip_ansi,
};

pub(crate) fn run_commands(commands: CommandsRef<'_>) -> Result<(), ()> {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let run_dir =
        std::path::Path::new(constants_str::WORKSPACE_TEST_RUNNER_RESULT_ROOT).join(format!(
            "{timestamp}-{}-{}",
            std::process::id(),
            RUN_COUNTER.fetch_add(1u64, std::sync::atomic::Ordering::Relaxed)
        ));
    std::fs::create_dir_all(run_dir.as_path())
        .map_err(ExecutionIoError::from)
        .map_err(|error| {
            eprintln!("failed to create test result directory: {}", error.0);
        })?;
    let mut command_runs = std::thread::scope(|scope| {
        commands
            .0
            .iter()
            .enumerate()
            .map(|(idx, (program, args))| {
                scope.spawn(move || {
                    let started_at = CommandStartedAtInstant::from(std::time::Instant::now());
                    let output = macro_helpers::domain_types::tool_command::ToolCommand::new(
                        macro_helpers::domain_types::tool_command::ToolProgramRef::from(*program),
                    )
                    .args(macro_helpers::domain_types::tool_command::ToolArgsRef::from(*args))
                    .output();
                    let (status_text, log_text, succeeded) = match output {
                        Ok(command_output) => {
                            let stdout = String::from_utf8_lossy(command_output.stdout.as_slice());
                            let stderr = String::from_utf8_lossy(command_output.stderr.as_slice());
                            print!("{stdout}");
                            eprint!("{stderr}");
                            (
                                command_output.status.to_string(),
                                format!("{stdout}{stderr}"),
                                command_output.status.success(),
                            )
                        }
                        Err(error) => (
                            format!("spawn-error:{error}"),
                            format!("failed to spawn command: {error}\n"),
                            false,
                        ),
                    };
                    CommandRun {
                        idx: CommandIdx::from(idx),
                        log_text: CommandText::try_from(log_text).unwrap_or_else(CommandText::from),
                        status_text: CommandText::try_from(status_text)
                            .unwrap_or_else(CommandText::from),
                        succeeded: CommandSucceeded::from(succeeded),
                        duration: started_at.elapsed(),
                    }
                })
            })
            .collect::<Vec<_>>()
            .into_iter()
            .map(std::thread::ScopedJoinHandle::join)
            .collect::<Vec<_>>()
    });
    let mut summary = SummaryText::try_from(String::new()).map_err(|_error| ())?;
    let mut succeeded = true;
    command_runs.sort_by_key(|command_run_result| match command_run_result {
        Ok(command_run) => command_run.idx.get(),
        Err(_panic) => usize::MAX,
    });
    command_runs.into_iter().try_for_each(|command_run_result| {
        let command_run = match command_run_result {
            Ok(command_run) => command_run,
            Err(_panic) => {
                succeeded = false;
                summary.push_str(TextRef::from(
                    constants_str::COMMAND_THREAD_PANICKED_SUMMARY,
                ))?;
                return Ok(());
            }
        };
        let (program, args) = commands
            .0
            .get(command_run.idx.get())
            .copied()
            .ok_or(())?;
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
        let log_name = CommandText::try_from(format!(
            "{:02}-{sanitized}.log",
            command_run.idx.get()
        ))
        .unwrap_or_else(CommandText::from);
        let log_path = run_dir.join(log_name.as_ref());
        if let Err(error) = std::fs::write(log_path.as_path(), command_run.log_text.as_ref()) {
            eprintln!(
                "failed to write test result log {}: {}",
                log_path.display(),
                error
            );
            return Err(());
        }
        let failed_test_names = failed_test_names(TextRef::from(command_run.log_text.as_ref()));
        let failed_names_capacity = failed_test_names
            .0
            .iter()
            .map(|name| name.as_ref().len())
            .sum::<usize>()
            .saturating_add(
                failed_test_names
                    .0
                    .len()
                    .get()
                    .saturating_sub(constants_usize::ONE)
                    .saturating_mul(constants_str::TEXT_ALT_7.len()),
            );
        let failed_names = failed_test_names.0.iter().enumerate().fold(
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
            TextRef::from(
                format!(
                "command={program} args={args:?} duration_ms={} status={} log={} failed_tests={failed_names}\n",
                command_run.duration.as_millis(),
                command_run.status_text.as_ref(),
                log_path.display()
            )
            .as_str(),
            ),
        )?;
        if !command_run.succeeded.get() {
            succeeded = false;
        }
        Ok(())
    })?;
    std::fs::write(
        run_dir.join(constants_str::SUMMARY_TXT),
        strip_ansi(TextRef::from(summary.0.as_str())).as_ref(),
    )
    .map_err(ExecutionIoError::from)
    .map_err(|error| {
        eprintln!("failed to write test result summary: {}", error.0);
    })?;
    if succeeded { Ok(()) } else { Err(()) }
}
