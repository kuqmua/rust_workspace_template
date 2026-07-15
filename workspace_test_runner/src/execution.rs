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
        .join(str_constants::expr::S_0048);
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
        std::path::Path::new(str_constants::workspace_test_runner::RESULT_ROOT).join(format!(
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
            line.strip_prefix(str_constants::expr::S_1801)
                .and_then(|tail| tail.strip_suffix(str_constants::expr::S_0006))
                .or_else(|| {
                    let tail = line.strip_prefix(str_constants::expr::S_0000)?;
                    tail.strip_suffix(str_constants::expr::S_0005)
                })
                .map(str::to_owned)
        })
        .collect::<Vec<String>>();
    names.sort();
    names.dedup();
    names
}
fn write_summary(run_dir: &RunDir, summary: &SummaryText) -> Result<(), std::io::Error> {
    std::fs::write(
        run_dir.0.join(str_constants::expr::S_1778),
        strip_ansi(summary.0.as_str()),
    )
}
pub(super) fn run_commands(commands: &[(&str, &[&str])]) -> Result<(), ()> {
    let run_dir = create_run_dir().map_err(|error| {
        super::reporting::result_directory_failed(&error);
    })?;
    let result = commands.iter().enumerate().try_fold(
        SummaryText(String::new()),
        |mut summary, (idx, (program, args))| {
            let started_at = CommandStartedAt(std::time::Instant::now());
            let output = macros_helpers::tool_command::ToolCommand::new(
                macros_helpers::tool_command::ToolProgramRef::from(*program),
            )
                .args(macros_helpers::tool_command::ToolArgsRef::from(*args))
                .output();
            let log_name = command_log_name(CommandIdx(idx), program, args);
            let log_path = run_dir.0.join(log_name.as_str());
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
            if let Err(error) = std::fs::write(log_path.as_path(), log_text.as_bytes()) {
                super::reporting::result_log_failed(log_path.as_path(), &error);
                return Err(summary);
            }
            let failed_names = failed_test_names(log_text.as_str()).join(str_constants::expr::S_0046);
            summary.0.push_str(
                format!(
                    "command={program} args={args:?} duration_ms={} status={status_text} log={} failed_tests={failed_names}\n",
                    started_at.elapsed().as_millis(),
                    log_path.display()
                )
                .as_str(),
            );
            if succeeded {
                Ok(summary)
            } else {
                Err(summary)
            }
        },
    );
    let summary = match result {
        Ok(summary) => summary,
        Err(summary) => {
            if let Err(error) = write_summary(&run_dir, &summary) {
                super::reporting::result_summary_failed(&error);
            }
            return Err(());
        }
    };
    write_summary(&run_dir, &summary).map_err(|error| {
        super::reporting::result_summary_failed(&error);
    })
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
