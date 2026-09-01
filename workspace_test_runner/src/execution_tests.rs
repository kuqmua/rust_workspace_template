#[cfg(test)]
mod tests {
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
            ["crate::domain_types::first", "crate::domain_types::second"]
        );
    }
    #[test]
    fn test_failed_test_parser_parses_partial_log() {
        assert!(
            crate::failed_test_names::failed_test_names(crate::text_ref::TextRef::from(
                "test incomplete"
            ))
            .as_ref()
            .is_empty()
        );
    }
    #[test]
    fn test_ansi_is_removed_from_machine_summary() {
        assert_eq!(
            crate::strip_ansi::strip_ansi(crate::text_ref::TextRef::from(
                "a\u{1b}[31mred\u{1b}[0mz"
            ))
            .as_ref(),
            "aredz"
        );
    }
}
