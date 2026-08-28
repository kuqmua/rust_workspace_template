pub(crate) use crate::command_duration::CommandDuration;
pub(crate) use crate::command_duration_millis::CommandDurationMillis;
pub(crate) use crate::command_idx::CommandIdx;
pub(crate) use crate::command_run::CommandRun;
pub(crate) use crate::command_started_at_instant::CommandStartedAtInstant;
pub(crate) use crate::command_succeeded::CommandSucceeded;
pub(crate) use crate::command_text::CommandText;
pub(crate) use crate::command_texts::CommandTexts;
pub(crate) use crate::commands_ref::CommandsRef;
pub(crate) use crate::execution_io_error::ExecutionIoError;
pub(crate) use crate::failed_test_names::failed_test_names;
pub(crate) use crate::run_commands::run_commands;
pub(crate) use crate::run_counter::RUN_COUNTER;
pub(crate) use crate::strip_ansi::strip_ansi;
pub(crate) use crate::summary_text::SummaryText;
pub(crate) use crate::text_ref::TextRef;

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
