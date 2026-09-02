#[cfg(test)]
mod tests {
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
        let Err(_error) =
            crate::env_var_result_var_error::EnvVarResultVarError::try_from(Ok(value))
        else {
            std::panic::panic_any(constants_str::PANIC_3EE39BCB);
        };
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
    }
}
