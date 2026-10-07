#[cfg(test)]
mod tests {
    #[test]
    fn test_tracing_format_accepts_only_json_and_text() {
        let parse = |value: &str| {
            <crate::tracing_format::TracingFormat as crate::try_from_std_env_var_ok::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                crate::std_env_var_ok::StdEnvVarOk::try_from(String::from(value))
                    .expect(constants_str::DIAGNOSTIC_9C260DCE),
            )
        };
        assert_eq!(
            parse(constants_str::JSON),
            Ok(crate::tracing_format::TracingFormat::Json)
        );
        assert_eq!(
            parse(&constants_str::JSON.to_ascii_uppercase()),
            Ok(crate::tracing_format::TracingFormat::Json)
        );
        assert_eq!(
            parse(constants_str::TRACING_FORMAT_TEXT),
            Ok(crate::tracing_format::TracingFormat::Text)
        );
        assert_eq!(
            parse(&constants_str::TRACING_FORMAT_TEXT.to_ascii_uppercase()),
            Ok(crate::tracing_format::TracingFormat::Text)
        );
        assert_eq!(
            crate::tracing_format::TracingFormat::default(),
            crate::tracing_format::TracingFormat::Text
        );
        [constants_str::JSON, constants_str::TRACING_FORMAT_TEXT]
            .into_iter()
            .fold((), |(), format| {
                [
                    format!("{}{format}", constants_str::SPACE),
                    format!("{format}{}", constants_str::SPACE),
                ]
                .into_iter()
                .fold((), |(), text| {
                    assert_eq!(
                        parse(&text),
                        Err(crate::try_from_std_env_var_ok_tracing_format_error::TryFromStdEnvVarOkTracingFormatError::Unknown)
                    );
                });
            });
        [constants_str::EMPTY, constants_str::SPACE]
            .into_iter()
            .fold((), |(), text| {
                assert_eq!(
                    parse(text),
                    Err(crate::try_from_std_env_var_ok_tracing_format_error::TryFromStdEnvVarOkTracingFormatError::Unknown)
                );
            });
        assert_eq!(
            parse(constants_str::BAD),
            Err(crate::try_from_std_env_var_ok_tracing_format_error::TryFromStdEnvVarOkTracingFormatError::Unknown)
        );
    }
    fn env_result(
        result: Result<String, std::env::VarError>,
    ) -> crate::env_var_result_var_error::EnvVarResultVarError {
        crate::env_var_result_var_error::EnvVarResultVarError::try_from(result)
            .expect(constants_str::DIAGNOSTIC_A4AA0C6F)
    }
    #[test]
    fn test_environment_result_rejects_values_above_shared_limit() {
        let value = constants_str::TEST_JWT_SECRET_CHARACTER_A.repeat(
            crate::config_lib_string_wrapper_max_len::CONFIG_LIB_STRING_WRAPPER_MAX_LEN
                .saturating_add(constants_usize::ONE),
        );
        let Err(error) = crate::env_var_result_var_error::EnvVarResultVarError::try_from(Ok(value))
        else {
            std::panic::panic_any(constants_str::PANIC_3EE39BCB);
        };
        assert_eq!(
            error,
            crate::config_lib_string_wrapper_try_from_string_error::ConfigLibStringWrapperTryFromStringError::TooLong {
                len: crate::config_lib_string_wrapper_max_len::CONFIG_LIB_STRING_WRAPPER_MAX_LEN
                    .saturating_add(constants_usize::ONE),
                max: crate::config_lib_string_wrapper_max_len::CONFIG_LIB_STRING_WRAPPER_MAX_LEN,
            }
        );
        let env_parse_error = crate::env_parse_error::EnvParseError::from(error);
        assert!(std::error::Error::source(&env_parse_error).is_some_and(|source| {
            source.downcast_ref::<crate::config_lib_string_wrapper_try_from_string_error::ConfigLibStringWrapperTryFromStringError>()
                == Some(&error)
        }));
        assert!(matches!(
            env_parse_error,
            crate::env_parse_error::EnvParseError::ValueTooLong { source } if source == error
        ));
    }
    fn assert_parse_display_roundtrip_variants<T>()
    where
        T: Copy
            + Eq
            + std::fmt::Debug
            + std::fmt::Display
            + std::str::FromStr<Err = String>
            + strum::IntoEnumIterator,
    {
        assert!(
            T::iter().all(|value| {
                let name = value.to_string();
                T::from_str(&name) == Ok(value)
            }),
            "7d39b6f2"
        );
    }
    #[test]
    fn test_tracing_level_display_is_stable() {
        [
            (
                crate::tracing_level::TracingLevel::Trace,
                constants_str::CONFIG_TRACING_TRACE,
            ),
            (
                crate::tracing_level::TracingLevel::Debug,
                constants_str::CONFIG_TRACING_DEBUG,
            ),
            (
                crate::tracing_level::TracingLevel::Info,
                constants_str::CONFIG_TRACING_INFO,
            ),
            (
                crate::tracing_level::TracingLevel::Warn,
                constants_str::CONFIG_TRACING_WARN,
            ),
            (
                crate::tracing_level::TracingLevel::Error,
                constants_str::CONFIG_TRACING_ERROR,
            ),
        ]
        .into_iter()
        .fold((), |(), (tracing_level, name)| {
            assert_eq!(tracing_level.to_string(), name);
            assert_eq!(
                name.parse::<crate::tracing_level::TracingLevel>(),
                Ok(tracing_level)
            );
            assert_eq!(
                name.to_ascii_uppercase()
                    .parse::<crate::tracing_level::TracingLevel>(),
                Ok(tracing_level)
            );
        });
        assert_eq!(
            crate::tracing_level::TracingLevel::default(),
            crate::tracing_level::TracingLevel::Error
        );
        assert_parse_display_roundtrip_variants::<crate::tracing_level::TracingLevel>();
    }
    #[test]
    fn test_tracing_level_from_str_is_case_insensitive() {
        assert_eq!(
            <crate::tracing_level::TracingLevel as std::str::FromStr>::from_str(
                constants_str::TRACE
            ),
            Ok(crate::tracing_level::TracingLevel::Trace)
        );
        assert_eq!(
            <crate::tracing_level::TracingLevel as std::str::FromStr>::from_str(
                constants_str::VALUE_8448814D
            ),
            Ok(crate::tracing_level::TracingLevel::Warn)
        );
        let _error =
            <crate::tracing_level::TracingLevel as std::str::FromStr>::from_str(constants_str::BAD)
                .expect_err(constants_str::VALUE_9F8D72A1);
    }
    #[test]
    fn test_tracing_level_roundtrip_is_stable_for_all_variants() {
        assert_parse_display_roundtrip_variants::<crate::tracing_level::TracingLevel>();
    }
    #[test]
    fn test_source_place_type_from_str_roundtrip_is_stable_for_all_variants() {
        assert_parse_display_roundtrip_variants::<crate::source_place_type::SourcePlaceType>();
    }
    #[test]
    fn test_source_place_type_from_str_accepts_src_value() {
        assert_eq!(
            <crate::source_place_type::SourcePlaceType as std::str::FromStr>::from_str(
                constants_str::SRC_ALT
            ),
            Ok(crate::source_place_type::SourcePlaceType::Src)
        );
    }
    #[test]
    fn test_source_place_type_from_str_rejects_unknown_value() {
        let _error = <crate::source_place_type::SourcePlaceType as std::str::FromStr>::from_str(
            constants_str::BAD,
        )
        .expect_err(constants_str::VALUE_8D6F70BB);
    }
    #[test]
    fn test_source_place_type_default_is_github() {
        assert_eq!(
            crate::source_place_type::SourcePlaceType::default(),
            crate::source_place_type::SourcePlaceType::Github
        );
    }
    #[test]
    fn test_source_place_type_parse_error_contains_expected_context() {
        let error = <crate::source_place_type::SourcePlaceType as std::str::FromStr>::from_str(
            constants_str::UNKNOWN_ALT,
        )
        .expect_err(constants_str::F2CC7D6B);
        assert!(error.contains(constants_str::VALUE_21D222E0));
        assert!(error.contains(constants_str::VALUE_0C0C9A7B));
    }
    #[test]
    fn test_parse_source_place_type_env_value_parses_case_insensitively() {
        let parsed = crate::parse_from_str_with_context_tests::parse_from_str_with_context::<
            crate::source_place_type::SourcePlaceType,
        >(
            crate::env_var_value_ref::EnvVarValueRef::from(constants_str::GITHUB),
            crate::parse_context_ref::ParseContextRef::from(
                constants_str::CONFIG_SOURCE_PLACE_TYPE_PARSE_CONTEXT,
            ),
        );
        assert_eq!(
            parsed,
            Ok(crate::source_place_type::SourcePlaceType::Github)
        );
    }
    #[test]
    fn test_parse_source_place_type_env_value_wraps_parse_context() {
        let error = crate::parse_from_str_with_context_tests::parse_from_str_with_context::<
            crate::source_place_type::SourcePlaceType,
        >(
            crate::env_var_value_ref::EnvVarValueRef::from(constants_str::BAD),
            crate::parse_context_ref::ParseContextRef::from(
                constants_str::CONFIG_SOURCE_PLACE_TYPE_PARSE_CONTEXT,
            ),
        )
        .expect_err(constants_str::VALUE_8C9F2A17);
        let error_text = error.to_string();
        assert!(error_text.contains(constants_str::VALUE_0D5C69DF));
        assert!(error_text.contains(constants_str::VALUE_862F630D));
    }
    #[test]
    fn test_parse_from_env_var_with_wraps_missing_var_context() {
        let parsed = crate::parse_from_env_var_with_tests::parse_from_env_var_with(
            env_result(Err(std::env::VarError::NotPresent)),
            crate::parse_env_var_name_ref::ParseEnvVarNameRef::from(
                constants_str::ENV_NAMES_SOURCE_PLACE_TYPE,
            ),
            |_v| Ok(()),
        );
        let error = parsed.expect_err(constants_str::D2F3B74A);
        assert!(error.to_string().contains(constants_str::VALUE_48766AEF));
        assert_eq!(
            std::error::Error::source(&error)
                .expect(constants_str::DIAGNOSTIC_E6FBFE6B)
                .to_string(),
            constants_str::VALUE_0833BC56
        );
    }
    #[test]
    fn test_parse_from_env_var_with_passes_value_into_parse_callback() {
        let parsed = crate::parse_from_env_var_with_tests::parse_from_env_var_with(
            env_result(Ok(String::from(constants_str::SRC_ALT))),
            crate::parse_env_var_name_ref::ParseEnvVarNameRef::from(
                constants_str::ENV_NAMES_SOURCE_PLACE_TYPE,
            ),
            |v| Ok(v.as_ref().to_owned()),
        );
        assert_eq!(parsed, Ok(String::from(constants_str::SRC_ALT)));
    }
    #[test]
    fn test_environment_read_errors_preserve_source_and_skip_parser() {
        [
            std::env::VarError::NotPresent,
            std::env::VarError::NotUnicode(std::ffi::OsString::from(constants_str::BAD)),
        ]
        .into_iter()
        .fold((), |(), var_error| {
            let expected_diagnostic = var_error.to_string();
            let parser_invoked = std::cell::Cell::new(false);
            let parsed = crate::parse_from_env_var_with_tests::parse_from_env_var_with(
                env_result(Err(var_error)),
                crate::parse_env_var_name_ref::ParseEnvVarNameRef::from(
                    constants_str::ENV_NAMES_SOURCE_PLACE_TYPE,
                ),
                |_env_var_value_ref| {
                    parser_invoked.set(true);
                    Ok(())
                },
            );
            assert!(!parser_invoked.get());
            assert!(parsed.is_err_and(|env_parse_error| {
                assert!(
                    std::error::Error::source(&env_parse_error).is_some_and(|source| {
                        source.is::<crate::env_var_error::EnvVarError>()
                            && source.to_string() == expected_diagnostic
                    })
                );
                matches!(
                    env_parse_error,
                    crate::env_parse_error::EnvParseError::Read { name, source }
                        if name.to_string() == constants_str::ENV_NAMES_SOURCE_PLACE_TYPE
                            && source.to_string() == expected_diagnostic
                )
            }));
        });
    }
    #[test]
    fn test_parse_from_env_var_from_str_parses_bool_when_input_is_valid() {
        let parsed = crate::parse_from_env_var_from_str_tests::parse_from_env_var_from_str::<bool>(
            env_result(Ok(String::from(constants_str::TRUE))),
            crate::parse_env_var_name_ref::ParseEnvVarNameRef::from(
                constants_str::ENV_NAMES_SOURCE_PLACE_TYPE,
            ),
            crate::parse_context_ref::ParseContextRef::from(constants_str::BOOL_PARSE),
        );
        assert_eq!(parsed, Ok(true));
    }
    #[test]
    fn test_parse_from_env_var_from_str_wraps_context_when_parse_fails() {
        let error = crate::parse_from_env_var_from_str_tests::parse_from_env_var_from_str::<bool>(
            env_result(Ok(String::from(constants_str::X))),
            crate::parse_env_var_name_ref::ParseEnvVarNameRef::from(
                constants_str::ENV_NAMES_SOURCE_PLACE_TYPE,
            ),
            crate::parse_context_ref::ParseContextRef::from(constants_str::BOOL_PARSE),
        )
        .expect_err(constants_str::VALUE_7E4B3F19);
        assert!(error.to_string().contains(constants_str::VALUE_3461D5B9));
        assert!(constants_str::X.parse::<bool>().is_err_and(|source| matches!(
            &error,
            crate::env_parse_error::EnvParseError::Parse { context, detail }
                if *context == crate::parse_context_ref::ParseContextRef::from(constants_str::BOOL_PARSE)
                    && detail.as_ref() == source.to_string()
        )));
    }
    #[test]
    fn test_parse_source_place_type_from_env_var_wraps_missing_var_context() {
        let error =
            crate::source_place_type::SourcePlaceType::parse_source_place_type_from_env_var(
                env_result(Err(std::env::VarError::NotPresent)),
            )
            .expect_err(constants_str::VALUE_5A83F2BE);
        assert!(error.to_string().contains(constants_str::VALUE_48766AEF));
    }
    #[test]
    fn test_parse_source_place_type_from_env_var_parses_ok_value() {
        let parsed =
            crate::source_place_type::SourcePlaceType::parse_source_place_type_from_env_var(
                env_result(Ok(String::from(constants_str::SRC_ALT))),
            );
        assert_eq!(parsed, Ok(crate::source_place_type::SourcePlaceType::Src));
    }
    #[test]
    fn test_parse_from_str_with_context_parses_value_when_input_is_valid() {
        let parsed = crate::parse_from_str_with_context_tests::parse_from_str_with_context::<bool>(
            crate::env_var_value_ref::EnvVarValueRef::from(constants_str::TRUE),
            crate::parse_context_ref::ParseContextRef::from(constants_str::BOOL_PARSE),
        );
        assert_eq!(parsed, Ok(true));
    }
    #[test]
    fn test_parse_from_str_with_context_wraps_context_when_parsing_fails() {
        let error = crate::parse_from_str_with_context_tests::parse_from_str_with_context::<bool>(
            crate::env_var_value_ref::EnvVarValueRef::from(constants_str::X),
            crate::parse_context_ref::ParseContextRef::from(constants_str::BOOL_PARSE),
        )
        .expect_err(constants_str::VALUE_13FE8A6D);
        assert!(error.to_string().contains(constants_str::VALUE_3461D5B9));
        assert!(constants_str::X.parse::<bool>().is_err_and(|source| matches!(
            &error,
            crate::env_parse_error::EnvParseError::Parse { context, detail }
                if *context == crate::parse_context_ref::ParseContextRef::from(constants_str::BOOL_PARSE)
                    && detail.as_ref() == source.to_string()
        )));
    }
}
