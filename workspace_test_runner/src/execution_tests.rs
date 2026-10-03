#[cfg(test)]
mod tests {
    #[test]
    fn test_failed_test_parser_handles_ansi_colored_log() {
        let colored_log = format!(
            "{}{}{}",
            constants_str::WORKSPACE_TEST_RUNNER_ANSI_RED,
            constants_str::VALUE_E6CA5E47,
            constants_str::WORKSPACE_TEST_RUNNER_ANSI_RESET
        );
        let names = crate::failed_test_names::failed_test_names(crate::text_ref::TextRef::from(
            colored_log.as_str(),
        ));
        assert_eq!(
            names
                .as_ref()
                .iter()
                .map(crate::command_text::CommandText::as_ref)
                .collect::<Vec<&str>>(),
            [constants_str::VALUE_6B4D91DC, constants_str::VALUE_B40C5E30]
        );
    }

    #[test]
    fn test_failed_test_parser_parses_cargo_and_nextest_lines() {
        let names = crate::failed_test_names::failed_test_names(crate::text_ref::TextRef::from(
            constants_str::VALUE_E6CA5E47,
        ));
        assert_eq!(
            names
                .as_ref()
                .iter()
                .map(crate::command_text::CommandText::as_ref)
                .collect::<Vec<&str>>(),
            [constants_str::VALUE_6B4D91DC, constants_str::VALUE_B40C5E30]
        );
    }
    #[test]
    fn test_failed_test_parser_parses_partial_log() {
        assert!(
            crate::failed_test_names::failed_test_names(crate::text_ref::TextRef::from(
                constants_str::VALUE_95EB9084
            ))
            .as_ref()
            .is_empty()
        );
    }
    #[test]
    fn test_failed_test_parser_sorts_and_deduplicates_across_log_formats() {
        let cargo_second = format!(
            "{}{}{}",
            constants_str::TEST_ALT,
            constants_str::VALUE_B40C5E30,
            constants_str::FAILED_ALT,
        );
        let nextest_first = format!(
            "{}{}{}",
            constants_str::FOUR_SPACES,
            constants_str::VALUE_6B4D91DC,
            constants_str::FAILED,
        );
        let cargo_first = format!(
            "{}{}{}",
            constants_str::TEST_ALT,
            constants_str::VALUE_6B4D91DC,
            constants_str::FAILED_ALT,
        );
        let log = [
            cargo_second.as_str(),
            nextest_first.as_str(),
            cargo_second.as_str(),
            cargo_first.as_str(),
        ]
        .join(constants_str::NEWLINE);
        let names = crate::failed_test_names::failed_test_names(crate::text_ref::TextRef::from(
            log.as_str(),
        ));
        assert!(
            names
                .as_ref()
                .iter()
                .map(crate::command_text::CommandText::as_ref)
                .eq([constants_str::VALUE_6B4D91DC, constants_str::VALUE_B40C5E30,])
        );
    }

    #[test]
    fn test_failed_test_parser_requires_complete_line_markers() {
        [
            format!(
                "{}{}",
                constants_str::VALUE_6B4D91DC,
                constants_str::FAILED_ALT
            ),
            format!(
                "{}{}",
                constants_str::TEST_ALT,
                constants_str::VALUE_6B4D91DC
            ),
            format!(
                "{}{}",
                constants_str::FOUR_SPACES,
                constants_str::VALUE_6B4D91DC
            ),
            format!(
                "{}{}{}{}",
                constants_str::TEST_ALT,
                constants_str::VALUE_6B4D91DC,
                constants_str::FAILED_ALT,
                constants_str::X
            ),
        ]
        .into_iter()
        .fold((), |(), log| {
            assert!(
                crate::failed_test_names::failed_test_names(crate::text_ref::TextRef::from(
                    log.as_str()
                ))
                .as_ref()
                .is_empty()
            );
        });
    }
}
