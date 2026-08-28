#[test]
fn svc_mode_accepts_only_documented_values() {
    assert_eq!(
        <crate::domain_types::types::SvcMode as crate::domain_types::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
            crate::domain_types::StdEnvVarOk::try_from(constants_str::SERVICE_MODE_MIGRATE.to_owned())
                .expect("39a8e94f svc_mode_accepts_only_documented_values invariant must hold"),
        ),
        Ok(crate::domain_types::types::SvcMode::Migrate)
    );
    assert_eq!(
        <crate::domain_types::types::SvcMode as crate::domain_types::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
            crate::domain_types::StdEnvVarOk::try_from(constants_str::SERVICE_MODE_SERVE.to_owned())
                .expect("045ca5a1 svc_mode_accepts_only_documented_values invariant must hold"),
        ),
        Ok(crate::domain_types::types::SvcMode::Serve)
    );
    assert_eq!(
        <crate::domain_types::types::SvcMode as crate::domain_types::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
            crate::domain_types::StdEnvVarOk::try_from(constants_str::INVALID_REQUEST.to_owned())
                .expect("156cc47b svc_mode_accepts_only_documented_values invariant must hold"),
        ),
        Err(crate::domain_types::TryFromStdEnvVarOkSvcModeError::Unknown)
    );
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, PartialEq, Eq)]
enum ParseRequiredEnvVarTestError {
    EnvVar {
        env_var_name: crate::domain_types::EnvVarName,
    },
    Parse {
        parse: &'static str,
    },
}
fn parse_env<T>(v: &str) -> Result<T, T::Error>
where
    T: crate::domain_types::TryFromStdEnvVarOk,
{
    T::try_from_std_env_var_ok(
        crate::domain_types::StdEnvVarOk::try_from(v.to_owned())
            .unwrap_or_else(crate::domain_types::StdEnvVarOk::from),
    )
}
#[test]
fn administrator_token_text_deserialization_uses_bounded_try_from() {
    let issuer_deserializer = serde::de::value::StringDeserializer::<serde::de::value::Error>::new(
        constants_str::TEST_JWT_SECRET_CHARACTER_A.repeat(257usize),
    );
    let audience_deserializer =
        serde::de::value::StringDeserializer::<serde::de::value::Error>::new(
            constants_str::TEST_JWT_SECRET_CHARACTER_A.repeat(257usize),
        );
    let Err(_issuer_error) =
        <crate::domain_types::AdminTokenIssuer as serde::Deserialize>::deserialize(
            issuer_deserializer,
        )
    else {
        panic!("b286db7c");
    };
    let Err(_audience_error) =
        <crate::domain_types::AdminTokenAudience as serde::Deserialize>::deserialize(
            audience_deserializer,
        )
    else {
        panic!("70f1e49f");
    };
}
#[test]
fn cors_allow_origin_parsing_returns_value() {
    config_lib_macros::assert_parse_ok_matches!(
        crate::domain_types::CorsAllowOrigin,
        constants_str::ASTERISK,
        crate::domain_types::CorsAllowOrigin(_)
    );
}
#[test]
fn cors_allow_origin_parsing_returns_error_for_empty_string() {
    config_lib_macros::assert_empty_parse_err_matches!(
        crate::domain_types::CorsAllowOrigin,
        crate::domain_types::TryFromStdEnvVarOkCorsAllowOriginError::IsEmpty { .. }
    );
}
#[test]
fn database_url_parsing_returns_value_for_non_empty_input() {
    config_lib_macros::assert_parse_ok_matches!(
        crate::domain_types::DatabaseUrl,
        constants_str::POSTGRES_DB,
        crate::domain_types::DatabaseUrl(_)
    );
}
#[test]
fn database_url_parsing_returns_error_for_empty_string() {
    config_lib_macros::assert_empty_parse_err_matches!(
        crate::domain_types::DatabaseUrl,
        crate::domain_types::TryFromStdEnvVarOkDatabaseUrlError::IsEmpty { .. }
    );
}
#[test]
fn secret_url_debug_output_redacts_credentials() {
    let all_redacted = [
        constants_str::POSTGRES_USERNAME_LOCALHOST_TEST,
        constants_str::POSTGRES_USERNAME_PASSWORD_LOCALHOST_TEST_QUESTION_SSLMODE_DISABLE,
        constants_str::POSTGRES_PERCENT_PERCENT_40NAME_PERCENT_PERCENT_2FPASSWORD_PATH_1_TEST_FRAGMENT,
    ]
    .into_iter()
    .all(|raw| {
        let value = parse_env::<crate::domain_types::DatabaseUrl>(raw).expect("ae91f62c secret_url_debug_output_redacts_credentials invariant must hold");
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
fn mongo_url_parsing_returns_value_for_non_empty_input() {
    config_lib_macros::assert_parse_ok_matches!(
        crate::domain_types::MongoUrl,
        constants_str::MONGODB_DB,
        crate::domain_types::MongoUrl(_)
    );
}
#[test]
fn mongo_url_parsing_returns_error_for_empty_string() {
    config_lib_macros::assert_empty_parse_err_matches!(
        crate::domain_types::MongoUrl,
        crate::domain_types::TryFromStdEnvVarOkMongoUrlError::IsEmpty { .. }
    );
}
#[test]
fn redis_url_parsing_returns_value_for_non_empty_input() {
    config_lib_macros::assert_parse_ok_matches!(
        crate::domain_types::RedisUrl,
        constants_str::REDIS_DB,
        crate::domain_types::RedisUrl(_)
    );
}
#[test]
fn redis_url_parsing_returns_error_for_empty_string() {
    config_lib_macros::assert_empty_parse_err_matches!(
        crate::domain_types::RedisUrl,
        crate::domain_types::TryFromStdEnvVarOkRedisUrlError::IsEmpty { .. }
    );
}
#[test]
fn src_place_type_parsing_is_case_insensitive() {
    config_lib_macros::assert_parse_ok_matches!(
        crate::domain_types::SrcPlaceType,
        constants_str::GITHUB_ALT,
        crate::domain_types::SrcPlaceType(crate::domain_types::types::SrcPlaceType::Github)
    );
}
#[test]
fn src_place_type_parsing_returns_error_for_unknown_value() {
    config_lib_macros::assert_parse_err_matches!(
        crate::domain_types::SrcPlaceType,
        constants_str::BAD,
        crate::domain_types::TryFromStdEnvVarOkSrcPlaceTypeError::AppStateSrcPlaceTypeParsing { .. }
    );
}
#[test]
fn tracing_level_parsing_is_case_insensitive() {
    config_lib_macros::assert_parse_ok_matches!(
        crate::domain_types::TracingLevel,
        constants_str::DEBUG,
        crate::domain_types::TracingLevel(crate::domain_types::types::TracingLevel::Debug)
    );
}
#[test]
fn tracing_level_parsing_returns_error_for_unknown_value() {
    config_lib_macros::assert_parse_err_matches!(
        crate::domain_types::TracingLevel,
        constants_str::BAD,
        crate::domain_types::TryFromStdEnvVarOkTracingLevelError::AppStateTracingLevelParsing { .. }
    );
}
#[test]
fn enable_api_git_commit_check_parsing_returns_bool() {
    config_lib_macros::assert_parse_ok_matches!(
        crate::domain_types::EnableApiGitCommitCheck,
        constants_str::TRUE,
        crate::domain_types::EnableApiGitCommitCheck(true)
    );
}
#[test]
fn enable_api_git_commit_check_parsing_returns_error_for_invalid_bool() {
    config_lib_macros::assert_parse_err_matches!(
        crate::domain_types::EnableApiGitCommitCheck,
        constants_str::TRUTHY,
        crate::domain_types::TryFromStdEnvVarOkEnableApiGitCommitCheckError::BoolParsing { .. }
    );
}
#[test]
fn maximum_size_of_http_body_in_bytes_parsing_returns_usize() {
    let parsed = parse_env::<crate::domain_types::MaximumSizeOfHttpBodyInBytes>(
        constants_str::VALUE_128,
    )
    .expect(
        "d5b7a09e maximum_size_of_http_body_in_bytes_parsing_returns_usize invariant must hold",
    );
    assert_eq!(*parsed, 128usize);
}
#[test]
fn maximum_size_of_http_body_in_bytes_parsing_returns_error_for_invalid_number() {
    config_lib_macros::assert_parse_err_matches!(
        crate::domain_types::MaximumSizeOfHttpBodyInBytes,
        constants_str::VALUE_1K,
        crate::domain_types::TryFromStdEnvVarOkMaximumSizeOfHttpBodyInBytesError::UsizeParsing { .. }
    );
}
#[test]
fn maximum_size_of_http_body_in_bytes_parsing_returns_error_for_zero() {
    config_lib_macros::assert_parse_err_matches!(
        crate::domain_types::MaximumSizeOfHttpBodyInBytes,
        constants_str::VALUE_0,
        crate::domain_types::TryFromStdEnvVarOkMaximumSizeOfHttpBodyInBytesError::MaximumSizeOfHttpBodyInBytes { .. }
    );
}
#[test]
fn pg_pool_max_connections_parsing_returns_u32() {
    let parsed = parse_env::<crate::domain_types::PgPoolMaxConnections>(constants_str::VALUE_10)
        .expect("5d9032ac pg_pool_max_connections_parsing_returns_u32 invariant must hold");
    assert_eq!(*parsed, 10u32);
}
#[test]
fn pg_pool_max_connections_parsing_returns_error_for_invalid_number() {
    config_lib_macros::assert_parse_err_matches!(
        crate::domain_types::PgPoolMaxConnections,
        constants_str::BAD,
        crate::domain_types::TryFromStdEnvVarOkPgPoolMaxConnectionsError::U32Parsing { .. }
    );
}
#[test]
fn pg_pool_max_connections_parsing_returns_error_for_zero() {
    config_lib_macros::assert_parse_err_matches!(
        crate::domain_types::PgPoolMaxConnections,
        constants_str::VALUE_0,
        crate::domain_types::TryFromStdEnvVarOkPgPoolMaxConnectionsError::PgPoolMaxConnections { .. }
    );
}
#[test]
fn non_empty_string_parser_returns_error_for_empty_value() {
    config_lib_macros::assert_empty_parse_err_matches!(
        crate::domain_types::StartingCheckLink,
        crate::domain_types::TryFromStdEnvVarOkStartingCheckLinkError::IsEmpty { .. }
    );
}
#[test]
fn non_empty_string_parser_returns_value_for_non_empty_value() {
    config_lib_macros::assert_parse_ok_matches!(
        crate::domain_types::StartingCheckLink,
        constants_str::HTTPS_EXAMPLE_COM,
        crate::domain_types::StartingCheckLink(_)
    );
}
#[test]
fn service_socket_address_parsing_returns_socket_addr() {
    config_lib_macros::assert_parse_ok_matches!(
        crate::domain_types::ServiceSocketAddress,
        constants_str::VALUE_127_0_0_1_3000,
        crate::domain_types::ServiceSocketAddress(_)
    );
}
#[test]
fn service_socket_address_parsing_returns_error_for_invalid_addr() {
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
fn timezone_parsing_returns_timezone_for_valid_offset() {
    config_lib_macros::assert_parse_ok_matches!(
        crate::domain_types::ChronoTimezone,
        constants_str::VALUE_0,
        crate::domain_types::ChronoTimezone(_)
    );
}
#[test]
fn timezone_parsing_returns_i32_error_for_non_number() {
    config_lib_macros::assert_parse_err_matches!(
        crate::domain_types::ChronoTimezone,
        constants_str::NAN,
        crate::domain_types::TryFromStdEnvVarOkTimezoneError::I32Parsing { .. }
    );
}
#[test]
fn parse_east_fixed_offset_returns_offset_for_valid_seconds() {
    let parsed = crate::parse_east_fixed_offset(crate::TimezoneSeconds::from(3i32 * 3_600i32));
    assert!(matches!(parsed, Ok(v) if v.0.local_minus_utc() == 3i32 * 3_600i32));
}
#[test]
fn parse_east_fixed_offset_returns_error_for_out_of_range_seconds() {
    let parsed = crate::parse_east_fixed_offset(crate::TimezoneSeconds::from(i32::MAX));
    assert_eq!(
        parsed,
        Err(crate::domain_types::ChronoFixedOffsetError(
            constants_str::CONFIG_TIMEZONE_NOT_EAST_MSG,
        ))
    );
}
#[test]
fn timezone_parsing_returns_offset_error_when_out_of_range() {
    let out_of_range = i32::MAX.to_string();
    let error = parse_env::<crate::domain_types::ChronoTimezone>(&out_of_range);
    assert!(matches!(
        error,
        Err(crate::domain_types::TryFromStdEnvVarOkTimezoneError::ChronoFixedOffset { .. })
    ));
}
#[test]
fn parse_required_env_var_parses_value_when_env_var_exists() {
    let parsed = crate::domain_types::parse_required_env_var(
        crate::domain_types::EnvVarNameRef::from(constants_str::PATH_ALT),
        |_std_env_var_error, env_var_name| ParseRequiredEnvVarTestError::EnvVar { env_var_name },
        |v| Ok::<_, &'static str>(v.0.len()),
        |parse| ParseRequiredEnvVarTestError::Parse { parse },
    );
    assert!(matches!(parsed, Ok(v) if v > 0));
}
#[test]
fn parse_required_env_var_maps_missing_env_var_error() {
    let parsed = crate::domain_types::parse_required_env_var(
        crate::domain_types::EnvVarNameRef::from(constants_str::CONFIG_LIB_TEST_ENV_VAR_4E8A7F21),
        |_std_env_var_error, env_var_name| ParseRequiredEnvVarTestError::EnvVar { env_var_name },
        Ok::<_, &'static str>,
        |parse| ParseRequiredEnvVarTestError::Parse { parse },
    );
    assert_eq!(
        parsed,
        Err(ParseRequiredEnvVarTestError::EnvVar {
            env_var_name: crate::domain_types::EnvVarName::try_from(
                "CONFIG_LIB_TEST_ENV_VAR_4E8A7F21".to_owned()
            )
            .unwrap_or_else(crate::domain_types::EnvVarName::from)
        })
    );
}
#[test]
fn parse_required_env_var_maps_parse_error() {
    let parsed = crate::domain_types::parse_required_env_var(
        crate::domain_types::EnvVarNameRef::from(constants_str::PATH_ALT),
        |_std_env_var_error, env_var_name| ParseRequiredEnvVarTestError::EnvVar { env_var_name },
        |_v| Err::<(), _>(constants_str::PARSE_FAILED),
        |parse| ParseRequiredEnvVarTestError::Parse { parse },
    );
    assert_eq!(
        parsed,
        Err(ParseRequiredEnvVarTestError::Parse {
            parse: "parse failed"
        })
    );
}
