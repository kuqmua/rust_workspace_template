static RUN_COUNTER: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(constants_u64::ZERO);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
struct CommandIdx(usize);
impl CommandIdx {
    const fn get(self) -> usize {
        self.0
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
struct CommandStartedAtInstant(std::time::Instant);
impl CommandStartedAtInstant {
    fn elapsed(self) -> CommandDuration {
        CommandDuration::from(self.0.elapsed())
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
struct CommandDuration(std::time::Duration);
impl CommandDuration {
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
pub(crate) struct CommandsRef<'commands_lt>(
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
#[bounded_string(max = constants_usize::VALUE_16_777_216)]
struct CommandText(String);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
struct CommandTexts(
    bounded_types::domain_types::vector::BoundedVec<CommandText, 0, { usize::MAX }>,
);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error(transparent)]
struct ExecutionIoError(std::io::Error);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
struct TextRef<'text_lt>(&'text_lt str);
impl<'text_lt> TextRef<'text_lt> {
    const fn get(self) -> &'text_lt str {
        self.0
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
struct RunDirPathBuf(std::path::PathBuf);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::BoundedString, newtype::AsRefStr,
)]
#[bounded_string(max = constants_usize::VALUE_1_048_576)]
struct SummaryText(String);
impl SummaryText {
    fn push_str(&mut self, value: TextRef<'_>) -> Result<(), ()> {
        if self
            .0
            .len()
            .checked_add(value.get().len())
            .is_none_or(|len| len > constants_usize::VALUE_1_048_576)
        {
            return Err(());
        }
        self.0.push_str(value.get());
        Ok(())
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
struct CommandRun {
    duration: CommandDuration,
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
    CommandText::try_from(format!("{:02}-{sanitized}.log", idx.get()))
        .unwrap_or_else(CommandText::from)
}
#[allow(clippy::single_call_fn)] // unique run-directory construction has one filesystem owner
fn create_run_dir() -> Result<RunDirPathBuf, ExecutionIoError> {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let path =
        std::path::Path::new(constants_str::WORKSPACE_TEST_RUNNER_RESULT_ROOT).join(format!(
            "{timestamp}-{}-{}",
            std::process::id(),
            RUN_COUNTER.fetch_add(1u64, std::sync::atomic::Ordering::Relaxed)
        ));
    std::fs::create_dir_all(path.as_path()).map_err(ExecutionIoError::from)?;
    Ok(RunDirPathBuf::from(path))
}
#[allow(clippy::single_call_fn)] // log parsing stays independently unit-testable
fn failed_test_names(log_text: TextRef<'_>) -> CommandTexts {
    let mut names = log_text
        .get()
        .lines()
        .filter_map(|line| {
            line.strip_prefix(constants_str::TEST_ALT)
                .and_then(|tail| tail.strip_suffix(constants_str::FAILED_ALT))
                .or_else(|| {
                    let tail = line.strip_prefix(constants_str::FOUR_SPACES)?;
                    tail.strip_suffix(constants_str::FAILED)
                })
                .map(|name| {
                    CommandText::try_from(name.to_owned()).unwrap_or_else(CommandText::from)
                })
        })
        .collect::<Vec<CommandText>>();
    names.sort_by(|left, right| left.as_ref().cmp(right.as_ref()));
    names.dedup_by(|left, right| left.as_ref() == right.as_ref());
    CommandTexts::from(bounded_types::domain_types::vector::BoundedVec::from_max_iter(names))
}
#[allow(clippy::single_call_fn)] // summary persistence remains separate from command orchestration
fn write_summary(run_dir: &RunDirPathBuf, summary: &SummaryText) -> Result<(), ExecutionIoError> {
    std::fs::write(
        run_dir.0.join(constants_str::SUMMARY_TXT),
        strip_ansi(TextRef::from(summary.0.as_str())).as_ref(),
    )
    .map_err(ExecutionIoError::from)
}
pub(crate) fn run_commands(commands: CommandsRef<'_>) -> Result<(), ()> {
    let run_dir = create_run_dir().map_err(|error| {
        crate::adapters::reporting::result_directory_failed(
            crate::domain_types::RunnerIoErrorRef::from(&error.0),
        );
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
        let log_name = command_log_name(
            command_run.idx,
            CommandProgramRef::from(program),
            CommandArgsRef::from(args),
        );
        let log_path = run_dir.0.join(log_name.as_ref());
        if let Err(error) = std::fs::write(log_path.as_path(), command_run.log_text.as_ref()) {
            crate::adapters::reporting::result_log_failed(
                crate::domain_types::RunnerPathRef::from(log_path.as_path()),
                crate::domain_types::RunnerIoErrorRef::from(&error),
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
    write_summary(&run_dir, &summary).map_err(|error| {
        crate::adapters::reporting::result_summary_failed(
            crate::domain_types::RunnerIoErrorRef::from(&error.0),
        );
    })?;
    if succeeded { Ok(()) } else { Err(()) }
}
#[cfg(test)]
mod tests {
    #[test]
    fn failed_test_parser_handles_cargo_and_nextest_lines() {
        let names = super::failed_test_names(super::TextRef::from(
            "test crate::domain_types::first ... FAILED\n    crate::domain_types::second --- FAILED\nnot a failure\n",
        ));
        assert_eq!(
            names
                .0
                .iter()
                .map(super::CommandText::as_ref)
                .collect::<Vec<&str>>(),
            vec!["crate::domain_types::first", "crate::domain_types::second"]
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
