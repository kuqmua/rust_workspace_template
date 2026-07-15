static RUN_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0u64);
#[derive(Clone, Copy, Debug)]
struct CommandIdx(usize);
impl CommandIdx {
    const fn get(self) -> usize {
        self.0
    }
}
#[derive(Clone, Copy, Debug)]
struct CommandStartedAt(std::time::Instant);
impl CommandStartedAt {
    fn elapsed(self) -> std::time::Duration {
        self.0.elapsed()
    }
}
#[derive(Debug)]
struct RunDir(std::path::PathBuf);
#[derive(Debug)]
struct SummaryText(String);
#[derive(Debug)]
struct CommandRun {
    duration: std::time::Duration,
    idx: CommandIdx,
    log_text: String,
    status_text: String,
    succeeded: bool,
}
#[allow(clippy::single_call_fn)] // summary sanitization stays independently unit-testable
fn strip_ansi(value: &str) -> String {
    value
        .chars()
        .fold(
            (String::with_capacity(value.len()), false),
            |(mut output, escaping), character| match (escaping, character) {
                (true, 'm') => (output, false),
                (true, _) | (false, '\u{1b}') => (output, true),
                (false, _) => {
                    output.push(character);
                    (output, false)
                }
            },
        )
        .0
}
#[allow(clippy::single_call_fn)] // bounded artifact naming stays isolated from process execution
fn command_log_name(idx: CommandIdx, program: &str, args: &[&str]) -> String {
    let raw = std::iter::once(program)
        .chain(args.iter().copied())
        .take(3usize)
        .collect::<Vec<&str>>()
        .join(str_constants::HYPHEN);
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
    format!("{:02}-{sanitized}.log", idx.get())
}
#[allow(clippy::single_call_fn)] // unique run-directory construction has one filesystem owner
fn create_run_dir() -> Result<RunDir, std::io::Error> {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let path =
        std::path::Path::new(str_constants::WORKSPACE_TEST_RUNNER_RESULT_ROOT).join(format!(
            "{timestamp}-{}-{}",
            std::process::id(),
            RUN_COUNTER.fetch_add(1u64, std::sync::atomic::Ordering::Relaxed)
        ));
    std::fs::create_dir_all(path.as_path())?;
    Ok(RunDir(path))
}
#[allow(clippy::single_call_fn)] // log parsing stays independently unit-testable
fn failed_test_names(log_text: &str) -> Vec<String> {
    let mut names = log_text
        .lines()
        .filter_map(|line| {
            line.strip_prefix(str_constants::TEST_ALT)
                .and_then(|tail| tail.strip_suffix(str_constants::FAILED_ALT))
                .or_else(|| {
                    let tail = line.strip_prefix(str_constants::FOUR_SPACES)?;
                    tail.strip_suffix(str_constants::FAILED)
                })
                .map(str::to_owned)
        })
        .collect::<Vec<String>>();
    names.sort();
    names.dedup();
    names
}
#[allow(clippy::single_call_fn)] // summary persistence remains separate from command orchestration
fn write_summary(run_dir: &RunDir, summary: &SummaryText) -> Result<(), std::io::Error> {
    std::fs::write(
        run_dir.0.join(str_constants::SUMMARY_TXT),
        strip_ansi(summary.0.as_str()),
    )
}
pub(super) fn run_commands(commands: &[(&str, &[&str])]) -> Result<(), ()> {
    let run_dir = create_run_dir().map_err(|error| {
        super::reporting::result_directory_failed(&error);
    })?;
    let mut command_runs = std::thread::scope(|scope| {
        commands
            .iter()
            .enumerate()
            .map(|(idx, (program, args))| {
                scope.spawn(move || {
                    let started_at = CommandStartedAt(std::time::Instant::now());
                    let output = macros_helpers::tool_command::ToolCommand::new(
                        macros_helpers::tool_command::ToolProgramRef::from(*program),
                    )
                    .args(macros_helpers::tool_command::ToolArgsRef::from(*args))
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
                        idx: CommandIdx(idx),
                        log_text,
                        status_text,
                        succeeded,
                        duration: started_at.elapsed(),
                    }
                })
            })
            .collect::<Vec<_>>()
            .into_iter()
            .map(std::thread::ScopedJoinHandle::join)
            .collect::<Vec<_>>()
    });
    let mut summary = SummaryText(String::new());
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
                summary
                    .0
                    .push_str(str_constants::COMMAND_THREAD_PANICKED_SUMMARY);
                return Ok(());
            }
        };
        let (program, args) = commands
            .get(command_run.idx.get())
            .copied()
            .ok_or(())?;
        let log_name = command_log_name(command_run.idx, program, args);
        let log_path = run_dir.0.join(log_name.as_str());
        if let Err(error) = std::fs::write(log_path.as_path(), command_run.log_text.as_bytes()) {
            super::reporting::result_log_failed(log_path.as_path(), &error);
            return Err(());
        }
        let failed_names =
            failed_test_names(command_run.log_text.as_str()).join(str_constants::TEXT_ALT_7);
        summary.0.push_str(
            format!(
                "command={program} args={args:?} duration_ms={} status={} log={} failed_tests={failed_names}\n",
                command_run.duration.as_millis(),
                command_run.status_text,
                log_path.display()
            )
            .as_str(),
        );
        if !command_run.succeeded {
            succeeded = false;
        }
        Ok(())
    })?;
    write_summary(&run_dir, &summary).map_err(|error| {
        super::reporting::result_summary_failed(&error);
    })?;
    if succeeded { Ok(()) } else { Err(()) }
}
#[cfg(test)]
mod tests {
    #[test]
    fn failed_test_parser_handles_cargo_and_nextest_lines() {
        assert_eq!(
            super::failed_test_names(
                "test crate::first ... FAILED\n    crate::second --- FAILED\nnot a failure\n"
            ),
            vec!["crate::first".to_owned(), "crate::second".to_owned()]
        );
    }
    #[test]
    fn failed_test_parser_handles_partial_log() {
        assert!(super::failed_test_names("test incomplete").is_empty());
    }
    #[test]
    fn ansi_is_removed_from_machine_summary() {
        assert_eq!(super::strip_ansi("a\u{1b}[31mred\u{1b}[0mz"), "aredz");
    }
}
