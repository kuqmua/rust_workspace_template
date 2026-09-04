mod test_generate_validated_tokens_emits_validated_output {
    #[test]
    fn test_generate_validated_tokens_emits_validated_output() {
        let output = crate::generate_validated_tokens::generate_validated_tokens(
            2u8,
            |input| Ok::<u16, &str>(u16::from(input)),
            |parsed| Ok(parsed.saturating_add(3u16)),
            |built| Ok(built.saturating_mul(2u16)),
            |validated| validated.to_string(),
            String::from,
        );
        assert_eq!(output, constants_str::VALUE_10);
    }
}

mod test_generate_validated_tokens_stops_at_failed_stage {
    #[test]
    fn test_generate_validated_tokens_stops_at_failed_stage() {
        let output = crate::generate_validated_tokens::generate_validated_tokens(
            (),
            |()| Ok::<(), &str>(()),
            |()| Err(constants_str::CODE_STYLE_ERROR_ATTRIBUTE),
            |()| Ok(()),
            |()| constants_str::OK_ALT.to_owned(),
            String::from,
        );
        assert_eq!(output, constants_str::CODE_STYLE_ERROR_ATTRIBUTE);
    }
}
