#[test]
fn test_svc_mode_accepts_only_documented_values() {
    assert_eq!(
        <crate::svc_mode::SvcMode as crate::try_from_std_env_var_ok::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
            crate::std_env_var_ok::StdEnvVarOk::try_from(constants_str::SERVICE_MODE_MIGRATE.to_owned())
                .expect(constants_str::DIAGNOSTIC_39A8E94F),
        ),
        Ok(crate::svc_mode::SvcMode::Migrate)
    );
    assert_eq!(
        <crate::svc_mode::SvcMode as crate::try_from_std_env_var_ok::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
            crate::std_env_var_ok::StdEnvVarOk::try_from(constants_str::SERVICE_MODE_SERVE.to_owned())
                .expect(constants_str::DIAGNOSTIC_045CA5A1),
        ),
        Ok(crate::svc_mode::SvcMode::Serve)
    );
    assert_eq!(
        <crate::svc_mode::SvcMode as crate::try_from_std_env_var_ok::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
            crate::std_env_var_ok::StdEnvVarOk::try_from(constants_str::INVALID_REQUEST.to_owned())
                .expect(constants_str::DIAGNOSTIC_156CC47B),
        ),
        Err(crate::try_from_std_env_var_ok_svc_mode_error::TryFromStdEnvVarOkSvcModeError::Unknown)
    );
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, PartialEq, Eq)]
enum ParseRequiredEnvVarTestError {
    EnvVar {
        env_var_name: crate::env_var_name::EnvVarName,
    },
    Parse {
        parse: &'static str,
    },
}
fn parse_env<T>(v: &str) -> Result<T, T::Error>
where
    T: crate::try_from_std_env_var_ok::TryFromStdEnvVarOk,
{
    T::try_from_std_env_var_ok(
        crate::std_env_var_ok::StdEnvVarOk::try_from(v.to_owned())
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
    config_lib_macros::assert_empty_parse_err_matches!(
        crate::domain_types::CorsAllowOrigin,
        crate::domain_types::TryFromStdEnvVarOkCorsAllowOriginError::IsEmpty { .. }
    );
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
    config_lib_macros::assert_empty_parse_err_matches!(
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
    config_lib_macros::assert_empty_parse_err_matches!(
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
    config_lib_macros::assert_empty_parse_err_matches!(
        crate::domain_types::RedisUrl,
        crate::domain_types::TryFromStdEnvVarOkRedisUrlError::IsEmpty { .. }
    );
}
#[test]
fn test_src_place_type_parsing_is_case_insensitive() {
    let value = parse_env::<crate::domain_types::SrcPlaceType>(constants_str::GITHUB_ALT)
        .expect(constants_str::DIAGNOSTIC_F7D20B3A);
    assert_eq!(
        *value.get_inner(),
        crate::src_place_type::SrcPlaceType::Github
    );
}
#[test]
fn test_src_place_type_parsing_returns_error_for_unknown_value() {
    config_lib_macros::assert_parse_err_matches!(
        crate::domain_types::SrcPlaceType,
        constants_str::BAD,
        crate::domain_types::TryFromStdEnvVarOkSrcPlaceTypeError::AppStateSrcPlaceTypeParsing { .. }
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
    config_lib_macros::assert_parse_err_matches!(
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
    config_lib_macros::assert_parse_err_matches!(
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
    config_lib_macros::assert_parse_err_matches!(
        crate::maximum_size_of_http_body_in_bytes::MaximumSizeOfHttpBodyInBytes,
        constants_str::VALUE_1K,
        crate::try_from_std_env_var_ok_maximum_size_of_http_body_in_bytes_error::TryFromStdEnvVarOkMaximumSizeOfHttpBodyInBytesError::UsizeParsing { .. }
    );
}
#[test]
fn test_maximum_size_of_http_body_in_bytes_parsing_returns_error_for_zero() {
    config_lib_macros::assert_parse_err_matches!(
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
    config_lib_macros::assert_parse_err_matches!(
        crate::pg_pool_max_connections::PgPoolMaxConnections,
        constants_str::BAD,
        crate::try_from_std_env_var_ok_pg_pool_max_connections_error::TryFromStdEnvVarOkPgPoolMaxConnectionsError::U32Parsing { .. }
    );
}
#[test]
fn test_pg_pool_max_connections_parsing_returns_error_for_zero() {
    config_lib_macros::assert_parse_err_matches!(
        crate::pg_pool_max_connections::PgPoolMaxConnections,
        constants_str::VALUE_0,
        crate::try_from_std_env_var_ok_pg_pool_max_connections_error::TryFromStdEnvVarOkPgPoolMaxConnectionsError::PgPoolMaxConnections { .. }
    );
}
#[test]
fn test_non_empty_string_parser_returns_error_for_empty_value() {
    config_lib_macros::assert_empty_parse_err_matches!(
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
    config_lib_macros::assert_parse_err_matches!(
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
        Ok::<_, &'static str>,
        |parse| ParseRequiredEnvVarTestError::Parse { parse },
    );
    assert_eq!(
        parsed,
        Err(ParseRequiredEnvVarTestError::EnvVar {
            env_var_name: crate::env_var_name::EnvVarName::try_from(
                "CONFIG_LIB_TEST_ENV_VAR_4E8A7F21".to_owned()
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
