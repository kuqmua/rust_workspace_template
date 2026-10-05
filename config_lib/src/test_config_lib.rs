#[test]
fn test_service_mode_accepts_only_documented_values() {
    assert_eq!(
        <crate::service_mode::ServiceMode as crate::try_from_std_env_var_ok::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
            crate::std_env_var_ok::StdEnvVarOk::try_from(constants_str::SERVICE_MODE_MIGRATE.to_owned())
                .expect(constants_str::DIAGNOSTIC_39A8E94F),
        ),
        Ok(crate::service_mode::ServiceMode::Migrate)
    );
    assert_eq!(
        <crate::service_mode::ServiceMode as crate::try_from_std_env_var_ok::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
            crate::std_env_var_ok::StdEnvVarOk::try_from(constants_str::SERVICE_MODE_SERVE.to_owned())
                .expect(constants_str::DIAGNOSTIC_045CA5A1),
        ),
        Ok(crate::service_mode::ServiceMode::Serve)
    );
    assert_eq!(
        <crate::service_mode::ServiceMode as crate::try_from_std_env_var_ok::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
            crate::std_env_var_ok::StdEnvVarOk::try_from(constants_str::INVALID_REQUEST.to_owned())
                .expect(constants_str::DIAGNOSTIC_156CC47B),
        ),
        Err(crate::try_from_std_env_var_ok_service_mode_error::TryFromStdEnvVarOkServiceModeError::Unknown)
    );
}
#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, PartialEq, Eq)]
enum ParseRequiredEnvVarTestError {
    EnvVar {
        env_var_name: crate::env_var_name::EnvVarName,
    },
    Parse {
        parse: &'static str,
    },
    ValueTooLong {
        error: crate::config_lib_string_wrapper_try_from_string_error::ConfigLibStringWrapperTryFromStringError,
        env_var_name: crate::env_var_name::EnvVarName,
    },
}
fn parse_env<T>(str: &str) -> Result<T, T::Error>
where
    T: crate::try_from_std_env_var_ok::TryFromStdEnvVarOk,
{
    T::try_from_std_env_var_ok(
        crate::std_env_var_ok::StdEnvVarOk::try_from(str.to_owned())
            .unwrap_or_else(crate::std_env_var_ok::StdEnvVarOk::from),
    )
}
#[test]
fn test_administrator_token_text_deserialization_uses_bounded_try_from() {
    let issuer_deserializer = serde::de::value::StringDeserializer::<serde::de::value::Error>::new(
        constants_str::TEST_JWT_SECRET_CHARACTER_A.repeat(257usize),
    );
    let audience_deserializer =
        serde::de::value::StringDeserializer::<serde::de::value::Error>::new(
            constants_str::TEST_JWT_SECRET_CHARACTER_A.repeat(257usize),
        );
    let Err(_issuer_error) =
        <crate::admin_token_issuer::AdminTokenIssuer as serde::Deserialize>::deserialize(
            issuer_deserializer,
        )
    else {
        std::panic::panic_any(constants_str::PANIC_B286DB7C);
    };
    let Err(_audience_error) =
        <crate::admin_token_audience::AdminTokenAudience as serde::Deserialize>::deserialize(
            audience_deserializer,
        )
    else {
        std::panic::panic_any(constants_str::PANIC_70F1E49F);
    };
}
#[test]
fn test_cors_allow_origin_parsing_returns_value() {
    let value = parse_env::<crate::domain_types::CorsAllowOrigin>(constants_str::ASTERISK)
        .expect(constants_str::DIAGNOSTIC_3178AECC);
    assert_eq!(value.get_inner(), constants_str::ASTERISK);
}
#[test]
fn test_cors_allow_origin_parsing_returns_error_for_empty_string() {
    proc_macro_config_lib_assert_empty_parse_err_matches::assert_empty_parse_err_matches!(
        crate::domain_types::CorsAllowOrigin,
        crate::domain_types::TryFromStdEnvVarOkCorsAllowOriginError::IsEmpty { .. }
    );
}
#[test]
fn test_generated_non_empty_config_text_rejects_oversized_direct_string() {
    let oversized = constants_str::TEST_JWT_SECRET_CHARACTER_A.repeat(
        crate::config_lib_string_wrapper_max_len::CONFIG_LIB_STRING_WRAPPER_MAX_LEN
            + constants_usize::ONE,
    );
    assert!(matches!(
        crate::domain_types::CorsAllowOrigin::try_from(oversized.clone()),
        Err(crate::domain_types::TryFromStdEnvVarOkCorsAllowOriginError::TooLong)
    ));
    assert!(matches!(
        crate::domain_types::TrustedProxyRangesText::try_from(oversized.clone()),
        Err(crate::domain_types::TryFromStdEnvVarOkTrustedProxyRangesTextError::TooLong)
    ));
    assert!(matches!(
        crate::domain_types::StartingCheckLink::try_from(oversized),
        Err(crate::domain_types::TryFromStdEnvVarOkStartingCheckLinkError::TooLong)
    ));
}
#[test]
fn test_database_url_parsing_returns_value_for_non_empty_input() {
    drop(
        parse_env::<crate::domain_types::DatabaseUrl>(constants_str::POSTGRES_DB)
            .expect(constants_str::DIAGNOSTIC_19DE9EA2),
    );
}
#[test]
fn test_database_url_parsing_returns_error_for_empty_string() {
    proc_macro_config_lib_assert_empty_parse_err_matches::assert_empty_parse_err_matches!(
        crate::domain_types::DatabaseUrl,
        crate::domain_types::TryFromStdEnvVarOkDatabaseUrlError::IsEmpty { .. }
    );
}
#[test]
fn test_secret_url_debug_output_redacts_credentials() {
    let all_redacted = [
        constants_str::POSTGRES_USERNAME_LOCALHOST_TEST,
        constants_str::POSTGRES_USERNAME_PASSWORD_LOCALHOST_TEST_QUESTION_SSLMODE_DISABLE,
        constants_str::POSTGRES_PERCENT_PERCENT_40NAME_PERCENT_PERCENT_2FPASSWORD_PATH_1_TEST_FRAGMENT,
    ]
    .into_iter()
    .all(|raw| {
        let value = parse_env::<crate::domain_types::DatabaseUrl>(raw).expect(constants_str::DIAGNOSTIC_AE91F62C);
        let debug = format!("{value:?}");
        !debug.contains(raw)
            && !debug.contains(constants_str::USERNAME)
            && !debug.contains(constants_str::PASSWORD)
            && !debug.contains(constants_str::PERCENT_PERCENT_40NAME)
            && !debug.contains(constants_str::PERCENT_PERCENT_2FPASSWORD)
            && debug.contains(constants_str::REDACTED_ALT)
    });
    assert!(all_redacted);
}
#[test]
fn test_mongo_url_parsing_returns_value_for_non_empty_input() {
    drop(
        parse_env::<crate::domain_types::MongoUrl>(constants_str::MONGODB_DB)
            .expect(constants_str::DIAGNOSTIC_2FD74787),
    );
}
#[test]
fn test_mongo_url_parsing_returns_error_for_empty_string() {
    proc_macro_config_lib_assert_empty_parse_err_matches::assert_empty_parse_err_matches!(
        crate::domain_types::MongoUrl,
        crate::domain_types::TryFromStdEnvVarOkMongoUrlError::IsEmpty { .. }
    );
}
#[test]
fn test_redis_url_parsing_returns_value_for_non_empty_input() {
    drop(
        parse_env::<crate::domain_types::RedisUrl>(constants_str::REDIS_DB)
            .expect(constants_str::DIAGNOSTIC_A9F87D4F),
    );
}
#[test]
fn test_redis_url_parsing_returns_error_for_empty_string() {
    proc_macro_config_lib_assert_empty_parse_err_matches::assert_empty_parse_err_matches!(
        crate::domain_types::RedisUrl,
        crate::domain_types::TryFromStdEnvVarOkRedisUrlError::IsEmpty { .. }
    );
}
#[test]
fn test_source_place_type_parsing_is_case_insensitive() {
    let value = parse_env::<crate::domain_types::SourcePlaceType>(constants_str::GITHUB_ALT)
        .expect(constants_str::DIAGNOSTIC_F7D20B3A);
    assert_eq!(
        *value.get_inner(),
        crate::source_place_type::SourcePlaceType::Github
    );
}
#[test]
fn test_source_place_type_parsing_returns_error_for_unknown_value() {
    proc_macro_config_lib_assert_parse_err_matches::assert_parse_err_matches!(
        crate::domain_types::SourcePlaceType,
        constants_str::BAD,
        crate::domain_types::TryFromStdEnvVarOkSourcePlaceTypeError::AppStateSourcePlaceTypeParsing { .. }
    );
}
#[test]
fn test_tracing_level_parsing_is_case_insensitive() {
    let value = parse_env::<crate::domain_types::TracingLevel>(constants_str::DEBUG)
        .expect(constants_str::DIAGNOSTIC_1209EA21);
    assert_eq!(
        *value.get_inner(),
        crate::tracing_level::TracingLevel::Debug
    );
}
#[test]
fn test_tracing_level_parsing_returns_error_for_unknown_value() {
    proc_macro_config_lib_assert_parse_err_matches::assert_parse_err_matches!(
        crate::domain_types::TracingLevel,
        constants_str::BAD,
        crate::domain_types::TryFromStdEnvVarOkTracingLevelError::AppStateTracingLevelParsing { .. }
    );
}
#[test]
fn test_enable_api_git_commit_check_parsing_returns_bool() {
    let value = parse_env::<crate::domain_types::EnableApiGitCommitCheck>(constants_str::TRUE)
        .expect(constants_str::DIAGNOSTIC_EA443A2A);
    assert!(*value.get_inner());
}
#[test]
fn test_enable_api_git_commit_check_parsing_returns_error_for_invalid_bool() {
    proc_macro_config_lib_assert_parse_err_matches::assert_parse_err_matches!(
        crate::domain_types::EnableApiGitCommitCheck,
        constants_str::TRUTHY,
        crate::domain_types::TryFromStdEnvVarOkEnableApiGitCommitCheckError::BoolParsing { .. }
    );
}
#[test]
fn test_maximum_size_of_http_body_in_bytes_parsing_returns_usize() {
    let parsed =
        parse_env::<crate::maximum_size_of_http_body_in_bytes::MaximumSizeOfHttpBodyInBytes>(
            constants_str::VALUE_128,
        )
        .expect(constants_str::DIAGNOSTIC_D5B7A09E);
    assert_eq!(*parsed, 128usize);
}
#[test]
fn test_maximum_size_of_http_body_in_bytes_parsing_returns_error_for_invalid_number() {
    proc_macro_config_lib_assert_parse_err_matches::assert_parse_err_matches!(
        crate::maximum_size_of_http_body_in_bytes::MaximumSizeOfHttpBodyInBytes,
        constants_str::VALUE_1K,
        crate::try_from_std_env_var_ok_maximum_size_of_http_body_in_bytes_error::TryFromStdEnvVarOkMaximumSizeOfHttpBodyInBytesError::UsizeParsing { .. }
    );
}
#[test]
fn test_maximum_size_of_http_body_in_bytes_parsing_returns_error_for_zero() {
    proc_macro_config_lib_assert_parse_err_matches::assert_parse_err_matches!(
        crate::maximum_size_of_http_body_in_bytes::MaximumSizeOfHttpBodyInBytes,
        constants_str::VALUE_0,
        crate::try_from_std_env_var_ok_maximum_size_of_http_body_in_bytes_error::TryFromStdEnvVarOkMaximumSizeOfHttpBodyInBytesError::MaximumSizeOfHttpBodyInBytes { .. }
    );
}
#[test]
fn test_pg_pool_max_connections_parsing_returns_u32() {
    let parsed =
        parse_env::<crate::pg_pool_max_connections::PgPoolMaxConnections>(constants_str::VALUE_10)
            .expect(constants_str::DIAGNOSTIC_5D9032AC);
    assert_eq!(*parsed, 10u32);
}
#[test]
fn test_pg_pool_max_connections_parsing_returns_error_for_invalid_number() {
    proc_macro_config_lib_assert_parse_err_matches::assert_parse_err_matches!(
        crate::pg_pool_max_connections::PgPoolMaxConnections,
        constants_str::BAD,
        crate::try_from_std_env_var_ok_pg_pool_max_connections_error::TryFromStdEnvVarOkPgPoolMaxConnectionsError::U32Parsing { .. }
    );
}
#[test]
fn test_pg_pool_max_connections_parsing_returns_error_for_zero() {
    proc_macro_config_lib_assert_parse_err_matches::assert_parse_err_matches!(
        crate::pg_pool_max_connections::PgPoolMaxConnections,
        constants_str::VALUE_0,
        crate::try_from_std_env_var_ok_pg_pool_max_connections_error::TryFromStdEnvVarOkPgPoolMaxConnectionsError::PgPoolMaxConnections { .. }
    );
}
#[test]
fn test_non_empty_string_parser_returns_error_for_empty_value() {
    proc_macro_config_lib_assert_empty_parse_err_matches::assert_empty_parse_err_matches!(
        crate::domain_types::StartingCheckLink,
        crate::domain_types::TryFromStdEnvVarOkStartingCheckLinkError::IsEmpty { .. }
    );
}
#[test]
fn test_non_empty_string_parser_returns_value_for_non_empty_value() {
    let value =
        parse_env::<crate::domain_types::StartingCheckLink>(constants_str::HTTPS_EXAMPLE_COM)
            .expect(constants_str::DIAGNOSTIC_9AEE76D2);
    assert_eq!(value.get_inner(), constants_str::HTTPS_EXAMPLE_COM);
}
#[test]
fn test_service_socket_address_parsing_returns_socket_addr() {
    let _address =
        parse_env::<crate::domain_types::ServiceSocketAddress>(constants_str::VALUE_127_0_0_1_3000)
            .expect(constants_str::DIAGNOSTIC_A8B92BAC);
}
#[test]
fn test_service_socket_address_parsing_returns_error_for_invalid_addr() {
    let error =
        parse_env::<crate::domain_types::ServiceSocketAddress>(constants_str::VALUE_127_0_0_1);
    assert!(matches!(
        error,
        Err(
            crate::domain_types::TryFromStdEnvVarOkServiceSocketAddressError::StdNetSocketAddr { .. }
        )
    ));
}
#[test]
fn test_timezone_parsing_returns_timezone_for_valid_offset() {
    let parsed = parse_env::<crate::chrono_timezone::ChronoTimezone>(constants_str::VALUE_0);
    assert!(matches!(parsed, Ok(value) if value.local_minus_utc() == 0i32));
}
#[test]
fn test_timezone_parsing_returns_i32_error_for_non_number() {
    proc_macro_config_lib_assert_parse_err_matches::assert_parse_err_matches!(
        crate::chrono_timezone::ChronoTimezone,
        constants_str::NAN,
        crate::try_from_std_env_var_ok_timezone_error::TryFromStdEnvVarOkTimezoneError::I32Parsing { .. }
    );
}
#[test]
fn test_parse_east_fixed_offset_returns_offset_for_valid_seconds() {
    let parsed = crate::parse_east_fixed_offset::parse_east_fixed_offset(
        crate::timezone_seconds::TimezoneSeconds::from(3i32 * 3_600i32),
    );
    assert!(matches!(parsed, Ok(v) if v.local_minus_utc() == 3i32 * 3_600i32));
}
#[test]
fn test_parse_east_fixed_offset_returns_error_for_out_of_range_seconds() {
    let parsed = crate::parse_east_fixed_offset::parse_east_fixed_offset(
        crate::timezone_seconds::TimezoneSeconds::from(i32::MAX),
    );
    assert_eq!(
        parsed,
        Err(
            crate::chrono_fixed_offset_error::ChronoFixedOffsetError::from(
                constants_str::CONFIG_TIMEZONE_NOT_EAST_MSG,
            )
        )
    );
}
#[test]
fn test_timezone_parsing_returns_offset_error_when_out_of_range() {
    let out_of_range = i32::MAX.to_string();
    let error = parse_env::<crate::chrono_timezone::ChronoTimezone>(&out_of_range);
    assert!(matches!(
        error,
        Err(crate::try_from_std_env_var_ok_timezone_error::TryFromStdEnvVarOkTimezoneError::ChronoFixedOffset { .. })
    ));
}
#[test]
fn test_parse_required_env_var_parses_value_when_env_var_exists() {
    let parsed = crate::parse_required_env_var::parse_required_env_var(
        crate::env_var_name_ref::EnvVarNameRef::from(constants_str::PATH_ALT),
        |_std_env_var_error, env_var_name| ParseRequiredEnvVarTestError::EnvVar { env_var_name },
        |error, env_var_name| ParseRequiredEnvVarTestError::ValueTooLong {
            error,
            env_var_name,
        },
        |v| Ok::<_, &'static str>(v.len()),
        |parse| ParseRequiredEnvVarTestError::Parse { parse },
    );
    assert!(matches!(parsed, Ok(v) if v > 0));
}
#[test]
fn test_parse_required_env_var_maps_missing_env_var_error() {
    let parsed = crate::parse_required_env_var::parse_required_env_var(
        crate::env_var_name_ref::EnvVarNameRef::from(
            constants_str::CONFIG_LIB_TEST_ENV_VAR_4E8A7F21,
        ),
        |_std_env_var_error, env_var_name| ParseRequiredEnvVarTestError::EnvVar { env_var_name },
        |error, env_var_name| ParseRequiredEnvVarTestError::ValueTooLong {
            error,
            env_var_name,
        },
        Ok::<_, &'static str>,
        |parse| ParseRequiredEnvVarTestError::Parse { parse },
    );
    assert_eq!(
        parsed,
        Err(ParseRequiredEnvVarTestError::EnvVar {
            env_var_name: crate::env_var_name::EnvVarName::try_from(
                constants_str::CONFIG_LIB_TEST_ENV_VAR_4E8A7F21.to_owned()
            )
            .unwrap_or_else(crate::env_var_name::EnvVarName::from)
        })
    );
}
#[test]
fn test_parse_required_env_var_maps_parse_error() {
    let parsed = crate::parse_required_env_var::parse_required_env_var(
        crate::env_var_name_ref::EnvVarNameRef::from(constants_str::PATH_ALT),
        |_std_env_var_error, env_var_name| ParseRequiredEnvVarTestError::EnvVar { env_var_name },
        |error, env_var_name| ParseRequiredEnvVarTestError::ValueTooLong {
            error,
            env_var_name,
        },
        |_v| Err::<(), _>(constants_str::PARSE_FAILED),
        |parse| ParseRequiredEnvVarTestError::Parse { parse },
    );
    assert_eq!(
        parsed,
        Err(ParseRequiredEnvVarTestError::Parse {
            parse: constants_str::PARSE_FAILED
        })
    );
}
#[test]
fn test_parse_required_env_var_rejects_oversized_value_before_parsing() {
    let maximum_length =
        crate::config_lib_string_wrapper_max_len::CONFIG_LIB_STRING_WRAPPER_MAX_LEN;
    let mut parse_called = false;
    let parsed = crate::parse_required_env_var_value::parse_required_env_var_value(
        crate::std_env_var_ok::StdEnvVarOk::try_from(
            constants_str::TEST_JWT_SECRET_CHARACTER_A
                .repeat(maximum_length + constants_usize::ONE),
        ),
        crate::env_var_name_ref::EnvVarNameRef::from(constants_str::PATH_ALT),
        |error, env_var_name| ParseRequiredEnvVarTestError::ValueTooLong {
            error,
            env_var_name,
        },
        |_value| {
            parse_called = true;
            Ok::<_, &'static str>(())
        },
        |parse| ParseRequiredEnvVarTestError::Parse { parse },
    );
    assert_eq!(
        parsed,
        Err(ParseRequiredEnvVarTestError::ValueTooLong {
            error: crate::config_lib_string_wrapper_try_from_string_error::ConfigLibStringWrapperTryFromStringError::TooLong {
                len: maximum_length + constants_usize::ONE,
                max: maximum_length,
            },
            env_var_name: crate::env_var_name::EnvVarName::try_from(constants_str::PATH_ALT.to_owned())
                .unwrap_or_else(crate::env_var_name::EnvVarName::from),
        })
    );
    assert!(!parse_called);
}

#[test]
fn test_config_field_descriptor_preserves_parser_results_and_metadata() {
    assert!(
        [
            crate::config_field_sensitivity::ConfigFieldSensitivity::Public,
            crate::config_field_sensitivity::ConfigFieldSensitivity::Secret,
        ]
        .into_iter()
        .all(|config_field_sensitivity| {
            let descriptor = crate::config_field_descriptor::ConfigFieldDescriptor::new(
                crate::env_var_name_ref::EnvVarNameRef::from(constants_str::FIELD),
                crate::config_field_example_ref::ConfigFieldExampleRef::from(constants_str::X),
                |std_env_var_ok: crate::std_env_var_ok::StdEnvVarOk| {
                    if std_env_var_ok.as_str() == constants_str::X {
                        crate::config_example_validity::ConfigExampleValidity::Valid
                    } else {
                        crate::config_example_validity::ConfigExampleValidity::Invalid
                    }
                },
                crate::config_field_requirement::ConfigFieldRequirement::Required,
                crate::config_rust_type_name::ConfigRustTypeName::from(constants_str::X),
                config_field_sensitivity,
            );
            assert_eq!(descriptor.env_name().as_ref(), constants_str::FIELD);
            assert_eq!(descriptor.example().as_ref(), constants_str::X);
            assert_eq!(
                descriptor.requirement(),
                crate::config_field_requirement::ConfigFieldRequirement::Required
            );
            assert_eq!(descriptor.rust_type_name().as_ref(), constants_str::X);
            assert_eq!(descriptor.sensitivity(), config_field_sensitivity);
            assert!(
                crate::std_env_var_ok::StdEnvVarOk::try_from(constants_str::X.to_owned())
                    .is_ok_and(|std_env_var_ok| descriptor.validate_example(std_env_var_ok)
                        == crate::config_example_validity::ConfigExampleValidity::Valid)
            );
            assert!(
                crate::std_env_var_ok::StdEnvVarOk::try_from(constants_str::VALUE_1.to_owned())
                    .is_ok_and(|std_env_var_ok| descriptor.validate_example(std_env_var_ok)
                        == crate::config_example_validity::ConfigExampleValidity::Invalid)
            );
            let debug = format!("{descriptor:?}");
            debug.starts_with(constants_str::CONFIG_FIELD_DESCRIPTOR)
                && [
                    constants_str::ENV_NAME,
                    constants_str::EXAMPLE,
                    constants_str::REQUIRED,
                    constants_str::RUST_TYPE_NAME,
                    constants_str::SENSITIVITY,
                ]
                .into_iter()
                .all(|field| debug.contains(field))
                && debug.ends_with('}')
        })
    );
}

#[test]
fn test_config_string_wrappers_preserve_limits_and_error_diagnostics() {
    let maximum = crate::config_lib_string_wrapper_max_len::CONFIG_LIB_STRING_WRAPPER_MAX_LEN;
    assert!(
        crate::env_var_name::EnvVarName::try_from(String::new())
            .is_ok_and(|env_var_name| env_var_name.to_string().is_empty())
    );
    assert!(
        crate::std_env_var_ok::StdEnvVarOk::try_from(String::new())
            .is_ok_and(|std_env_var_ok| std_env_var_ok.is_empty())
    );
    assert!(
        crate::env_var_name::EnvVarName::try_from(constants_str::X.repeat(maximum))
            .is_ok_and(|env_var_name| env_var_name.to_string().len() == maximum)
    );
    assert!(
        crate::std_env_var_ok::StdEnvVarOk::try_from(constants_str::X.repeat(maximum))
            .is_ok_and(|std_env_var_ok| std_env_var_ok.len() == maximum)
    );
    let error = crate::config_lib_string_wrapper_try_from_string_error::ConfigLibStringWrapperTryFromStringError::TooLong {
        len: maximum + 1usize,
        max: maximum,
    };
    assert_eq!(
        crate::env_var_name::EnvVarName::try_from(constants_str::X.repeat(maximum + 1usize)),
        Err(error)
    );
    assert_eq!(
        crate::std_env_var_ok::StdEnvVarOk::try_from(constants_str::X.repeat(maximum + 1usize)),
        Err(error)
    );
    let error_text = error.to_string();
    assert_eq!(
        crate::env_var_name::EnvVarName::from(error).to_string(),
        error_text
    );
    assert_eq!(
        crate::std_env_var_ok::StdEnvVarOk::from(error).as_str(),
        error_text
    );
}

#[test]
fn test_content_security_policy_preserves_trimming_and_byte_boundaries() {
    let padded = [' ', '\t', 'x', ' ', '\r', '\n']
        .into_iter()
        .collect::<String>();
    assert!(
        crate::content_security_policy::ContentSecurityPolicy::try_from(padded).is_ok_and(
            |content_security_policy| content_security_policy.as_ref() == constants_str::X
        )
    );
    assert_eq!(
        crate::content_security_policy::ContentSecurityPolicy::try_from(String::new()),
        Err(crate::content_security_policy_error::ContentSecurityPolicyError::Empty)
    );
    assert_eq!(
        crate::content_security_policy::ContentSecurityPolicy::try_from(
            ['x', '\r', 'x'].into_iter().collect::<String>()
        ),
        Err(crate::content_security_policy_error::ContentSecurityPolicyError::Invalid)
    );
    assert_eq!(
        crate::content_security_policy::ContentSecurityPolicy::try_from(
            ['x', '\n', 'x'].into_iter().collect::<String>()
        ),
        Err(crate::content_security_policy_error::ContentSecurityPolicyError::Invalid)
    );
    assert!(
        crate::content_security_policy::ContentSecurityPolicy::try_from(
            constants_str::X.repeat(4_096usize)
        )
        .is_ok_and(|content_security_policy| content_security_policy.as_ref().len() == 4_096usize)
    );
    assert_eq!(
        crate::content_security_policy::ContentSecurityPolicy::try_from(
            constants_str::X.repeat(4_097usize)
        ),
        Err(crate::content_security_policy_error::ContentSecurityPolicyError::Invalid)
    );
    let multibyte_character = '\u{00e9}'.to_string();
    assert!(
        crate::content_security_policy::ContentSecurityPolicy::try_from(
            multibyte_character.repeat(2_048usize)
        )
        .is_ok_and(|content_security_policy| content_security_policy.as_ref().len() == 4_096usize)
    );
    assert_eq!(
        crate::content_security_policy::ContentSecurityPolicy::try_from(
            multibyte_character.repeat(2_049usize)
        ),
        Err(crate::content_security_policy_error::ContentSecurityPolicyError::Invalid)
    );
    assert!(crate::std_env_var_ok::StdEnvVarOk::try_from(constants_str::X.to_owned())
        .is_ok_and(|std_env_var_ok| <crate::content_security_policy::ContentSecurityPolicy as crate::try_from_std_env_var_ok::TryFromStdEnvVarOk>::try_from_std_env_var_ok(std_env_var_ok)
            .is_ok_and(|content_security_policy| content_security_policy.as_ref() == constants_str::X)));
    assert!(crate::std_env_var_ok::StdEnvVarOk::try_from(constants_str::NEWLINE.to_owned())
        .is_ok_and(|std_env_var_ok| <crate::content_security_policy::ContentSecurityPolicy as crate::try_from_std_env_var_ok::TryFromStdEnvVarOk>::try_from_std_env_var_ok(std_env_var_ok)
            == Err(crate::content_security_policy_error::ContentSecurityPolicyError::Empty)));
    assert!(crate::std_env_var_ok::StdEnvVarOk::try_from(['x', '\n', 'x'].into_iter().collect::<String>())
        .is_ok_and(|std_env_var_ok| <crate::content_security_policy::ContentSecurityPolicy as crate::try_from_std_env_var_ok::TryFromStdEnvVarOk>::try_from_std_env_var_ok(std_env_var_ok)
            == Err(crate::content_security_policy_error::ContentSecurityPolicyError::Invalid)));
}

#[test]
fn test_password_hash_concurrency_preserves_positive_values_and_parse_diagnostics() {
    let parse_concurrency = |std_env_var_ok| {
        <crate::admin_password_hash_concurrency::AdminPasswordHashConcurrency as crate::try_from_std_env_var_ok::TryFromStdEnvVarOk>::try_from_std_env_var_ok(std_env_var_ok)
    };
    assert!(
        [
            (constants_str::VALUE_1.to_owned(), 1usize),
            (['+', '1'].into_iter().collect::<String>(), 1usize),
            (
                std::num::NonZeroUsize::MAX.to_string(),
                std::num::NonZeroUsize::MAX.get()
            ),
        ]
        .into_iter()
        .all(
            |(text, expected)| crate::std_env_var_ok::StdEnvVarOk::try_from(text)
                .is_ok_and(|std_env_var_ok| parse_concurrency(std_env_var_ok)
                    .is_ok_and(|concurrency| concurrency.get() == expected))
        )
    );
    assert!(crate::std_env_var_ok::StdEnvVarOk::try_from(constants_str::VALUE_0.to_owned())
        .is_ok_and(|std_env_var_ok| parse_concurrency(std_env_var_ok).is_err_and(|error| matches!(error,
            crate::try_from_std_env_var_ok_admin_password_hash_concurrency_error::TryFromStdEnvVarOkAdminPasswordHashConcurrencyError::IsZero))));
    let mut overflow = std::num::NonZeroUsize::MAX.to_string();
    overflow.push('0');
    assert!([
        constants_str::X.to_owned(),
        String::new(),
        ['-', '1'].into_iter().collect::<String>(),
        [' ', '1'].into_iter().collect::<String>(),
        overflow,
    ].into_iter().all(|text| {
        let expected = text.parse::<usize>().err().map(|error| format!("{error:?}"));
        assert!(expected.is_some());
        crate::std_env_var_ok::StdEnvVarOk::try_from(text).is_ok_and(|std_env_var_ok| {
            parse_concurrency(std_env_var_ok).is_err_and(|error| {
                let crate::try_from_std_env_var_ok_admin_password_hash_concurrency_error::TryFromStdEnvVarOkAdminPasswordHashConcurrencyError::Parse { admin_positive_usize_parsing } = error else {
                    return false;
                };
                let observed = format!("{admin_positive_usize_parsing:?}");
                expected.as_deref().is_some_and(|diagnostic| observed == diagnostic)
            })
        })
    }));
}
