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
}
