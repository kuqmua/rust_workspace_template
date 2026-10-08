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

#[test]
fn test_validated_token_pipeline_preserves_callback_order_and_short_circuiting() {
    #[derive(
        Clone, Copy, Debug, Eq, PartialEq, proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    )]
    enum TestValidatedTokenPhase {
        Build,
        Emit,
        Error,
        Parse,
        Validate,
    }
    [
        (
            None,
            vec![
                TestValidatedTokenPhase::Parse,
                TestValidatedTokenPhase::Build,
                TestValidatedTokenPhase::Validate,
                TestValidatedTokenPhase::Emit,
            ],
        ),
        (
            Some(TestValidatedTokenPhase::Parse),
            vec![
                TestValidatedTokenPhase::Parse,
                TestValidatedTokenPhase::Error,
            ],
        ),
        (
            Some(TestValidatedTokenPhase::Build),
            vec![
                TestValidatedTokenPhase::Parse,
                TestValidatedTokenPhase::Build,
                TestValidatedTokenPhase::Error,
            ],
        ),
        (
            Some(TestValidatedTokenPhase::Validate),
            vec![
                TestValidatedTokenPhase::Parse,
                TestValidatedTokenPhase::Build,
                TestValidatedTokenPhase::Validate,
                TestValidatedTokenPhase::Error,
            ],
        ),
    ]
    .into_iter()
    .fold((), |(), (failure, expected)| {
        let phases = std::cell::RefCell::new(Vec::new());
        let run_phase = |test_validated_token_phase| {
            phases.borrow_mut().push(test_validated_token_phase);
            if failure == Some(test_validated_token_phase) {
                Err(constants_str::CODE_STYLE_ERROR_ATTRIBUTE)
            } else {
                Ok(())
            }
        };
        let output = crate::generate_validated_tokens::generate_validated_tokens(
            (),
            |()| run_phase(TestValidatedTokenPhase::Parse),
            |()| run_phase(TestValidatedTokenPhase::Build),
            |()| run_phase(TestValidatedTokenPhase::Validate),
            |()| {
                phases.borrow_mut().push(TestValidatedTokenPhase::Emit);
                constants_str::OK_ALT
            },
            |str| {
                phases.borrow_mut().push(TestValidatedTokenPhase::Error);
                str
            },
        );
        assert_eq!(*phases.borrow(), expected);
        assert_eq!(
            output,
            if failure.is_some() {
                constants_str::CODE_STYLE_ERROR_ATTRIBUTE
            } else {
                constants_str::OK_ALT
            }
        );
    });
}
