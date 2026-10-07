#[test]
fn test_location_text_helper_preserves_valid_text_and_bounded_fallbacks() {
    let maximum = crate::loc_test_text_max_len::LOC_TEST_TEXT_MAX_LEN;
    [
        constants_str::EMPTY.to_owned(),
        constants_str::X.to_owned(),
        constants_str::X.repeat(maximum),
        constants_str::X.repeat(maximum.saturating_add(1usize)),
        [
            constants_str::X
                .repeat(maximum.saturating_sub(constants_str::U_1F496.len()))
                .as_str(),
            constants_str::U_1F496,
        ]
        .concat(),
        [
            constants_str::X
                .repeat(
                    maximum
                        .saturating_sub(constants_str::U_1F496.len())
                        .saturating_add(1usize),
                )
                .as_str(),
            constants_str::U_1F496,
        ]
        .concat(),
    ]
    .into_iter()
    .fold((), |(), raw_value| {
        let deserializer =
            serde::de::value::StrDeserializer::<serde::de::value::Error>::new(&raw_value);
        let parsed =
            <crate::location_test_text::LocationTestText as serde::Deserialize>::deserialize(
                deserializer,
            );
        let expected = if raw_value.len() > maximum {
            assert!(parsed.is_err());
            crate::location_test_text::LocationTestText::from(
                crate::location_test_text::LocationTestTextTryFromStringError::TooLong {
                    len: raw_value.len(),
                    max: maximum,
                },
            )
        } else {
            parsed.unwrap_or_else(|error| std::panic::panic_any(error))
        };
        assert_eq!(
            crate::create_location_test_text::create_location_test_text(raw_value),
            expected
        );
    });
    [constants_str::EMPTY, constants_str::X]
        .into_iter()
        .fold((), |(), value| {
            let text = crate::location_test_text::LocationTestText::from(value);
            assert_eq!(
                to_err_string::to_err_string::ToErrString::to_err_string(&text).as_ref(),
                value
            );
        });
    #[allow(
        clippy::large_stack_arrays,
        reason = "inline const fixture bytes are promoted to immutable static data rather than allocated on the runtime stack"
    )]
    let oversized_static_text = const {
        std::str::from_utf8(
            &[b'x'; crate::loc_test_text_max_len::LOC_TEST_TEXT_MAX_LEN.saturating_add(1usize)],
        )
    }
    .unwrap_or_else(|error| std::panic::panic_any(error));
    let expected_static_fallback = crate::location_test_text::LocationTestText::from(
        crate::location_test_text::LocationTestTextTryFromStringError::TooLong {
            len: maximum.saturating_add(1usize),
            max: maximum,
        },
    );
    assert_eq!(
        crate::location_test_text::LocationTestText::from(oversized_static_text),
        expected_static_fallback
    );
}

#[test]
fn test_location_struct_diagnostics_preserve_debug_text_and_oversized_fallbacks() {
    fn location_diagnostic_matches<Diagnostic, Factory>(
        factory: Factory,
    ) -> crate::location_test_flag::LocationTestFlag
    where
        Diagnostic: std::fmt::Debug + to_err_string::to_err_string::ToErrString,
        Factory: Fn(crate::location_test_text::LocationTestText) -> Diagnostic,
    {
        crate::location_test_flag::LocationTestFlag::from(
            [
                constants_str::X.to_owned(),
                constants_str::X.repeat(crate::loc_test_text_max_len::LOC_TEST_TEXT_MAX_LEN),
                '\n'.to_string()
                    .repeat(crate::loc_test_text_max_len::LOC_TEST_TEXT_MAX_LEN),
            ]
            .into_iter()
            .all(|raw_value| {
                crate::location_test_text::LocationTestText::try_from(raw_value).is_ok_and(|text| {
                    let diagnostic = factory(text);
                    let debug = format!("{diagnostic:?}");
                    let actual =
                        to_err_string::to_err_string::ToErrString::to_err_string(&diagnostic);
                    match to_err_string::error_text::ErrorText::try_from(debug) {
                        Ok(expected) => actual.as_ref() == expected.as_ref(),
                        Err(error) => {
                            actual.as_ref()
                                == to_err_string::error_text::ErrorText::from(error).as_ref()
                        }
                    }
                })
            }),
        )
    }
    let display_matches = location_diagnostic_matches(|location_test_text| {
        crate::display_struct::DisplayStruct::new(
            location_test_text,
            crate::location_test_flag::LocationTestFlag::from(true),
        )
    });
    let serde_matches = location_diagnostic_matches(|location_test_text| {
        crate::serde_struct::SerdeStruct::new(
            location_test_text,
            crate::location_test_count::LocationTestCount::from(u32::MAX),
            crate::location_test_flag::LocationTestFlag::from(false),
        )
    });
    assert_eq!(
        display_matches,
        crate::location_test_flag::LocationTestFlag::from(true)
    );
    assert_eq!(
        serde_matches,
        crate::location_test_flag::LocationTestFlag::from(true)
    );
}
