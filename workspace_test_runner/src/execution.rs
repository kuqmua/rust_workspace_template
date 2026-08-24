static RUN_COUNTER: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(u64_constants::ZERO);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
struct CommandIdx(usize);
impl CommandIdx {
    const fn get(self) -> usize {
        self.0
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
struct StdCommandStartedAt(std::time::Instant);
impl StdCommandStartedAt {
    fn elapsed(self) -> StdCommandDuration {
        StdCommandDuration::from(self.0.elapsed())
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
struct StdCommandDuration(std::time::Duration);
impl StdCommandDuration {
    fn as_millis(self) -> CommandDurationMillis {
        CommandDurationMillis::from(self.0.as_millis())
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::Display,
    newtype::FromInner,
)]
struct CommandDurationMillis(u128);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
struct CommandSucceeded(bool);
impl CommandSucceeded {
    const fn get(self) -> bool {
        self.0
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
pub(super) struct CommandsRef<'commands_lt>(
    &'commands_lt [(&'commands_lt str, &'commands_lt [&'commands_lt str])],
);
impl<'commands_lt, const N: usize>
    From<&'commands_lt [(&'commands_lt str, &'commands_lt [&'commands_lt str]); N]>
    for CommandsRef<'commands_lt>
{
    fn from(
        value: &'commands_lt [(&'commands_lt str, &'commands_lt [&'commands_lt str]); N],
    ) -> Self {
        Self(value.as_slice())
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
struct CommandProgramRef<'program_lt>(&'program_lt str);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
struct CommandArgsRef<'args_lt>(&'args_lt [&'args_lt str]);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::AsRefStr, newtype::BoundedString,
)]
#[bounded_string(max = usize_constants::VALUE_16_777_216)]
struct CommandText(String);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
struct CommandTexts(bounded_types::BoundedVec<CommandText, 0, { usize::MAX }>);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error(transparent)]
struct StdExecutionIoError(std::io::Error);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
struct TextRef<'text_lt>(&'text_lt str);
impl<'text_lt> TextRef<'text_lt> {
    const fn get(self) -> &'text_lt str {
        self.0
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
struct StdRunDir(std::path::PathBuf);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::BoundedString, newtype::AsRefStr,
)]
#[bounded_string(max = usize_constants::VALUE_1_048_576)]
struct SummaryText(String);
impl SummaryText {
    fn push_str(&mut self, value: TextRef<'_>) -> Result<(), ()> {
        if self
            .0
            .len()
            .checked_add(value.get().len())
            .is_none_or(|len| len > usize_constants::VALUE_1_048_576)
        {
            return Err(());
        }
        self.0.push_str(value.get());
        Ok(())
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
struct CommandRun {
    duration: StdCommandDuration,
    idx: CommandIdx,
    log_text: CommandText,
    status_text: CommandText,
    succeeded: CommandSucceeded,
}
#[allow(clippy::single_call_fn)] // summary sanitization stays independently unit-testable
fn strip_ansi(value: TextRef<'_>) -> CommandText {
    let output = value
        .get()
        .chars()
        .fold(
            (String::with_capacity(value.get().len()), false),
            |(mut output, escaping), character| match (escaping, character) {
                (true, 'm') => (output, false),
                (true, _) | (false, '\u{1b}') => (output, true),
                (false, _) => {
                    output.push(character);
                    (output, false)
                }
            },
        )
        .0;
    CommandText::try_from(output).unwrap_or_else(CommandText::from)
}
#[allow(clippy::single_call_fn)] // bounded artifact naming stays isolated from process execution
fn command_log_name(
    idx: CommandIdx,
    program: CommandProgramRef<'_>,
    args: CommandArgsRef<'_>,
) -> CommandText {
    let parts = std::iter::once(program.0)
        .chain(args.0.iter().copied())
        .take(3usize);
    let raw_capacity = parts
        .clone()
        .map(str::len)
        .sum::<usize>()
        .saturating_add(parts.clone().count().saturating_sub(usize_constants::ONE));
    let raw = parts.enumerate().fold(
        String::with_capacity(raw_capacity),
        |mut raw, (index, part)| {
            if index > usize_constants::ZERO {
                raw.push_str(str_constants::HYPHEN);
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
    CommandText::try_from(format!("{:02}-{sanitized}.log", idx.get()))
        .unwrap_or_else(CommandText::from)
}
#[allow(clippy::single_call_fn)] // unique run-directory construction has one filesystem owner
fn create_run_dir() -> Result<StdRunDir, StdExecutionIoError> {
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
    std::fs::create_dir_all(path.as_path()).map_err(StdExecutionIoError::from)?;
    Ok(StdRunDir::from(path))
}
#[allow(clippy::single_call_fn)] // log parsing stays independently unit-testable
fn failed_test_names(log_text: TextRef<'_>) -> CommandTexts {
    let mut names = log_text
        .get()
        .lines()
        .filter_map(|line| {
            line.strip_prefix(str_constants::TEST_ALT)
                .and_then(|tail| tail.strip_suffix(str_constants::FAILED_ALT))
                .or_else(|| {
                    let tail = line.strip_prefix(str_constants::FOUR_SPACES)?;
                    tail.strip_suffix(str_constants::FAILED)
                })
                .map(|name| {
                    CommandText::try_from(name.to_owned()).unwrap_or_else(CommandText::from)
                })
        })
        .collect::<Vec<CommandText>>();
    names.sort_by(|left, right| left.as_ref().cmp(right.as_ref()));
    names.dedup_by(|left, right| left.as_ref() == right.as_ref());
    CommandTexts::from(bounded_types::BoundedVec::from_max_iter(names))
}
#[allow(clippy::single_call_fn)] // summary persistence remains separate from command orchestration
fn write_summary(run_dir: &StdRunDir, summary: &SummaryText) -> Result<(), StdExecutionIoError> {
    std::fs::write(
        run_dir.0.join(str_constants::SUMMARY_TXT),
        strip_ansi(TextRef::from(summary.0.as_str())).as_ref(),
    )
    .map_err(StdExecutionIoError::from)
}
pub(super) fn run_commands(commands: CommandsRef<'_>) -> Result<(), ()> {
    let run_dir = create_run_dir().map_err(|error| {
        super::reporting::result_directory_failed(super::StdRunnerIoErrorRef::from(&error.0));
    })?;
    let mut command_runs = std::thread::scope(|scope| {
        commands
            .0
            .iter()
            .enumerate()
            .map(|(idx, (program, args))| {
                scope.spawn(move || {
                    let started_at = StdCommandStartedAt::from(std::time::Instant::now());
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
                    str_constants::COMMAND_THREAD_PANICKED_SUMMARY,
                ))?;
                return Ok(());
            }
        };
        let (program, args) = commands
            .0
            .get(command_run.idx.get())
            .copied()
            .ok_or(())?;
        let log_name = command_log_name(
            command_run.idx,
            CommandProgramRef::from(program),
            CommandArgsRef::from(args),
        );
        let log_path = run_dir.0.join(log_name.as_ref());
        if let Err(error) = std::fs::write(log_path.as_path(), command_run.log_text.as_ref()) {
            super::reporting::result_log_failed(
                super::StdRunnerPathRef::from(log_path.as_path()),
                super::StdRunnerIoErrorRef::from(&error),
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
                    .saturating_sub(usize_constants::ONE)
                    .saturating_mul(str_constants::TEXT_ALT_7.len()),
            );
        let failed_names = failed_test_names.0.iter().enumerate().fold(
            String::with_capacity(failed_names_capacity),
            |mut names, (index, name)| {
                if index > usize_constants::ZERO {
                    names.push_str(str_constants::TEXT_ALT_7);
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
    write_summary(&run_dir, &summary).map_err(|error| {
        super::reporting::result_summary_failed(super::StdRunnerIoErrorRef::from(&error.0));
    })?;
    if succeeded { Ok(()) } else { Err(()) }
}
#[cfg(test)]
mod tests {
    #[test]
    fn failed_test_parser_handles_cargo_and_nextest_lines() {
        let names = super::failed_test_names(super::TextRef::from(
            "test crate::first ... FAILED\n    crate::second --- FAILED\nnot a failure\n",
        ));
        assert_eq!(
            names
                .0
                .iter()
                .map(super::CommandText::as_ref)
                .collect::<Vec<&str>>(),
            vec!["crate::first", "crate::second"]
        );
    }
    #[test]
    fn failed_test_parser_handles_partial_log() {
        assert!(
            super::failed_test_names(super::TextRef::from("test incomplete"))
                .0
                .is_empty()
        );
    }
    #[test]
    fn ansi_is_removed_from_machine_summary() {
        assert_eq!(
            super::strip_ansi(super::TextRef::from("a\u{1b}[31mred\u{1b}[0mz")).as_ref(),
            "aredz"
        );
    }
}
