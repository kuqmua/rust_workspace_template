#[path = "command_duration.rs"]
mod command_duration;
#[path = "command_duration_millis.rs"]
mod command_duration_millis;
#[path = "command_idx.rs"]
mod command_idx;
#[path = "command_run.rs"]
mod command_run;
#[path = "command_started_at_instant.rs"]
mod command_started_at_instant;
#[path = "command_succeeded.rs"]
mod command_succeeded;
#[path = "command_text.rs"]
mod command_text;
#[path = "command_texts.rs"]
mod command_texts;
#[path = "commands_ref.rs"]
mod commands_ref;
#[path = "execution_io_error.rs"]
mod execution_io_error;
#[path = "failed_test_names.rs"]
mod failed_test_names;
#[path = "run_commands.rs"]
mod run_commands;
#[path = "run_counter.rs"]
mod run_counter;
#[path = "strip_ansi.rs"]
mod strip_ansi;
#[path = "summary_text.rs"]
mod summary_text;
#[path = "text_ref.rs"]
mod text_ref;

use command_duration::CommandDuration;
use command_duration_millis::CommandDurationMillis;
use command_idx::CommandIdx;
use command_run::CommandRun;
use command_started_at_instant::CommandStartedAtInstant;
use command_succeeded::CommandSucceeded;
use command_text::CommandText;
use command_texts::CommandTexts;
pub(crate) use commands_ref::CommandsRef;
use execution_io_error::ExecutionIoError;
use failed_test_names::failed_test_names;
pub(crate) use run_commands::run_commands;
use run_counter::RUN_COUNTER;
use strip_ansi::strip_ansi;
use summary_text::SummaryText;
use text_ref::TextRef;

#[cfg(test)]
mod tests {
    #[test]
    fn failed_test_parser_parses_cargo_and_nextest_lines() {
        let names = super::failed_test_names(super::TextRef::from(constants_str::VALUE_E6CA5E47));
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
    fn failed_test_parser_parses_partial_log() {
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
