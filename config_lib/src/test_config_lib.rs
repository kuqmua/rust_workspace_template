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
    assert_eq!(
        crate::service_mode::ServiceMode::default(),
        crate::service_mode::ServiceMode::Serve
    );
    [constants_str::SERVICE_MODE_MIGRATE, constants_str::SERVICE_MODE_SERVE].into_iter().fold((), |(), name| {
        [name.to_ascii_uppercase(), format!("{}{}", constants_str::SPACE, name), format!("{}{}", name, constants_str::SPACE)]
            .into_iter().fold((), |(), text| {
                assert_eq!(parse_env::<crate::service_mode::ServiceMode>(&text), Err(crate::try_from_std_env_var_ok_service_mode_error::TryFromStdEnvVarOkServiceModeError::Unknown));
            });
    });
    assert_eq!(parse_env::<crate::service_mode::ServiceMode>(constants_str::EMPTY), Err(crate::try_from_std_env_var_ok_service_mode_error::TryFromStdEnvVarOkServiceModeError::Unknown));
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
    let unicode_text = format!(
        "{}{}",
        '\u{00e9}'.to_string().repeat(128usize),
        constants_str::X
    );
    [constants_str::TEST_JWT_SECRET_CHARACTER_A.repeat(257usize), unicode_text]
        .into_iter().fold((), |(), text| {
            let issuer_error = crate::admin_token_issuer::AdminTokenIssuerTryFromStringError::TooLong { len: 257usize, max: 256usize };
            let audience_error = crate::admin_token_audience::AdminTokenAudienceTryFromStringError::TooLong { len: 257usize, max: 256usize };
            assert_eq!(crate::admin_token_issuer::AdminTokenIssuer::try_from(text.clone()), Err(issuer_error));
            assert_eq!(crate::admin_token_audience::AdminTokenAudience::try_from(text.clone()), Err(audience_error));
            assert!(<crate::admin_token_issuer::AdminTokenIssuer as serde::Deserialize>::deserialize(
                serde::de::value::StrDeserializer::<serde::de::value::Error>::new(&text),
            ).is_err_and(|error| error.to_string() == issuer_error.to_string()));
            assert!(<crate::admin_token_audience::AdminTokenAudience as serde::Deserialize>::deserialize(
                serde::de::value::StrDeserializer::<serde::de::value::Error>::new(&text),
            ).is_err_and(|error| error.to_string() == audience_error.to_string()));
            assert!(parse_env::<crate::admin_token_issuer::AdminTokenIssuer>(&text).is_err_and(|error| error == crate::try_from_std_env_var_ok_admin_token_text_error::TryFromStdEnvVarOkAdminTokenTextError::TooLong));
            assert!(parse_env::<crate::admin_token_audience::AdminTokenAudience>(&text).is_err_and(|error| error == crate::try_from_std_env_var_ok_admin_token_text_error::TryFromStdEnvVarOkAdminTokenTextError::TooLong));
        });
}

#[test]
fn test_administrator_token_deserialization_preserves_complete_unicode_text() {
    [
        String::new(),
        constants_str::SPACE.to_owned(),
        ['"', '\\', '\n', '\u{00e9}']
            .into_iter()
            .collect::<String>(),
        '\u{00e9}'.to_string().repeat(128usize),
    ]
    .into_iter()
    .fold((), |(), text| {
        let issuer =
            <crate::admin_token_issuer::AdminTokenIssuer as serde::Deserialize>::deserialize(
                serde::de::value::StrDeserializer::<serde::de::value::Error>::new(&text),
            );
        let audience =
            <crate::admin_token_audience::AdminTokenAudience as serde::Deserialize>::deserialize(
                serde::de::value::StrDeserializer::<serde::de::value::Error>::new(&text),
            );
        assert!(issuer.is_ok() && audience.is_ok());
        let (Ok(issuer_value), Ok(audience_value)) = (issuer, audience) else {
            return;
        };
        assert_eq!(issuer_value.as_ref(), &text);
        assert_eq!(audience_value.as_ref(), &text);
        assert_eq!(issuer_value.as_bounded_string().as_str(), text);
        assert_eq!(audience_value.as_bounded_string().as_str(), text);
        if !text.is_empty() {
            assert!(
                parse_env::<crate::admin_token_issuer::AdminTokenIssuer>(&text)
                    .is_ok_and(|parsed| parsed == issuer_value)
            );
            assert!(
                parse_env::<crate::admin_token_audience::AdminTokenAudience>(&text)
                    .is_ok_and(|parsed| parsed == audience_value)
            );
        }
    });
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
fn test_trusted_proxy_text_preserves_content_and_unicode_byte_bounds() {
    let maximum = crate::config_lib_string_wrapper_max_len::CONFIG_LIB_STRING_WRAPPER_MAX_LEN;
    let character_count = maximum.checked_div(2usize);
    assert!(character_count.is_some());
    let Some(repetition_count) = character_count else {
        return;
    };
    assert_eq!(repetition_count.checked_mul(2usize), Some(maximum));
    [
        constants_str::SPACE.to_owned(),
        constants_str::VALUE_127_0_0_1.to_owned(),
        '\u{00e9}'.to_string().repeat(repetition_count),
    ]
    .into_iter()
    .fold((), |(), text| {
        assert!(
            crate::domain_types::TrustedProxyRangesText::try_from(text.clone())
                .is_ok_and(|value| value.get_inner().as_str() == text)
        );
        assert!(
            parse_env::<crate::domain_types::TrustedProxyRangesText>(&text)
                .is_ok_and(|value| value.get_inner().as_str() == text)
        );
    });
    let oversized = format!(
        "{}{}",
        '\u{00e9}'.to_string().repeat(repetition_count),
        constants_str::X
    );
    assert!(
        crate::domain_types::TrustedProxyRangesText::try_from(oversized).is_err_and(
            |error| matches!(
                error,
                crate::domain_types::TryFromStdEnvVarOkTrustedProxyRangesTextError::TooLong
            )
        )
    );
    assert!(crate::domain_types::TrustedProxyRangesText::try_from(String::new()).is_err_and(|error| matches!(error, crate::domain_types::TryFromStdEnvVarOkTrustedProxyRangesTextError::IsEmpty { is_empty } if is_empty == constants_str::CONFIG_ENV_VALUE_IS_EMPTY_MSG)));
    assert!(parse_env::<crate::domain_types::TrustedProxyRangesText>(constants_str::EMPTY).is_err_and(|error| matches!(error, crate::domain_types::TryFromStdEnvVarOkTrustedProxyRangesTextError::IsEmpty { is_empty } if is_empty == constants_str::CONFIG_ENV_VALUE_IS_EMPTY_MSG)));
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
fn test_secret_url_wrappers_preserve_exact_text_and_maximum_byte_length() {
    let maximum = crate::config_lib_string_wrapper_max_len::CONFIG_LIB_STRING_WRAPPER_MAX_LEN;
    let character_count = maximum.checked_div(2usize);
    assert!(character_count.is_some());
    let Some(repetition_count) = character_count else {
        return;
    };
    assert_eq!(repetition_count.checked_mul(2usize), Some(maximum));
    [
        constants_str::POSTGRES_USERNAME_PASSWORD_LOCALHOST_TEST_QUESTION_SSLMODE_DISABLE
            .to_owned(),
        constants_str::MONGODB_DB.to_owned(),
        constants_str::REDIS_DB.to_owned(),
        '\u{00e9}'.to_string().repeat(repetition_count),
    ]
    .into_iter()
    .fold((), |(), text| {
        assert!(
            parse_env::<crate::domain_types::DatabaseUrl>(&text).is_ok_and(|database_url| {
                secrecy::ExposeSecret::expose_secret(database_url.get_inner()).as_ref() == &text
                    && format!("{database_url:?}").contains(constants_str::REDACTED_ALT)
                    && !format!("{database_url:?}").contains(&text)
                    && format!("{database_url:#?}").contains(constants_str::REDACTED_ALT)
                    && !format!("{database_url:#?}").contains(&text)
            })
        );
        assert!(
            parse_env::<crate::domain_types::MongoUrl>(&text).is_ok_and(|mongo_url| {
                secrecy::ExposeSecret::expose_secret(mongo_url.get_inner()).as_ref() == &text
                    && format!("{mongo_url:?}").contains(constants_str::REDACTED_ALT)
                    && !format!("{mongo_url:?}").contains(&text)
                    && format!("{mongo_url:#?}").contains(constants_str::REDACTED_ALT)
                    && !format!("{mongo_url:#?}").contains(&text)
            })
        );
        assert!(
            parse_env::<crate::domain_types::RedisUrl>(&text).is_ok_and(|redis_url| {
                secrecy::ExposeSecret::expose_secret(redis_url.get_inner()).as_ref() == &text
                    && format!("{redis_url:?}").contains(constants_str::REDACTED_ALT)
                    && !format!("{redis_url:?}").contains(&text)
                    && format!("{redis_url:#?}").contains(constants_str::REDACTED_ALT)
                    && !format!("{redis_url:#?}").contains(&text)
            })
        );
    });
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
    assert!(
        parse_env::<crate::domain_types::EnableApiGitCommitCheck>(constants_str::FALSE)
            .is_ok_and(|disabled| !*disabled.get_inner())
    );
}
#[test]
fn test_enable_api_git_commit_check_parsing_returns_error_for_invalid_bool() {
    proc_macro_config_lib_assert_parse_err_matches::assert_parse_err_matches!(
        crate::domain_types::EnableApiGitCommitCheck,
        constants_str::TRUTHY,
        crate::domain_types::TryFromStdEnvVarOkEnableApiGitCommitCheckError::BoolParsing { .. }
    );
    [constants_str::EMPTY.to_owned(), constants_str::VALUE_1.to_owned(), constants_str::VALUE_0.to_owned(), constants_str::TRUE.to_ascii_uppercase(), format!("{}{}", constants_str::SPACE, constants_str::TRUE), format!("{}{}", constants_str::FALSE, constants_str::SPACE)]
        .into_iter().fold((), |(), text| {
            let native = text.parse::<bool>();
            assert!(native.is_err());
            let Err(source) = native else { return; };
            assert!(parse_env::<crate::domain_types::EnableApiGitCommitCheck>(&text).is_err_and(|error| {
                let crate::domain_types::TryFromStdEnvVarOkEnableApiGitCommitCheckError::BoolParsing { bool_parsing } = error;
                format!("{bool_parsing:?}") == format!("{source:?}")
            }));
        });
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
fn test_pool_maximum_connections_preserve_inclusive_bounds_and_zero_error() {
    [1u32, u32::MAX].into_iter().fold((), |(), value| {
        assert!(
            crate::pg_pool_max_connections::PgPoolMaxConnections::try_from(value)
                .is_ok_and(|maximum| *maximum == value)
        );
        assert!(
            parse_env::<crate::pg_pool_max_connections::PgPoolMaxConnections>(&value.to_string())
                .is_ok_and(|maximum| *maximum == value)
        );
    });
    assert!(crate::pg_pool_max_connections::PgPoolMaxConnections::try_from(0u32).is_err_and(|error| error == crate::pg_pool_max_connections_try_from_u32_error::PgPoolMaxConnectionsTryFromU32Error::IsZero));
    assert!(parse_env::<crate::pg_pool_max_connections::PgPoolMaxConnections>(constants_str::VALUE_0).is_err_and(|error| matches!(error, crate::try_from_std_env_var_ok_pg_pool_max_connections_error::TryFromStdEnvVarOkPgPoolMaxConnectionsError::PgPoolMaxConnections { pg_pool_max_connections } if pg_pool_max_connections == crate::pg_pool_max_connections_try_from_u32_error::PgPoolMaxConnectionsTryFromU32Error::IsZero)));
}

#[test]
fn test_pool_maximum_connections_preserve_native_parse_diagnostics() {
    [constants_str::EMPTY.to_owned(), constants_str::X.to_owned(), ['-', '1'].into_iter().collect::<String>(), [' ', '1'].into_iter().collect::<String>(), (u64::from(u32::MAX) + 1u64).to_string()]
        .into_iter().fold((), |(), text| {
            let expected = text.parse::<u32>();
            assert!(expected.is_err());
            let Err(source) = expected else { return; };
            assert!(parse_env::<crate::pg_pool_max_connections::PgPoolMaxConnections>(&text).is_err_and(|error| {
                let crate::try_from_std_env_var_ok_pg_pool_max_connections_error::TryFromStdEnvVarOkPgPoolMaxConnectionsError::U32Parsing { u32_parsing } = error else { return false; };
                format!("{u32_parsing:?}") == format!("{source:?}")
            }));
        });
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
    let address =
        parse_env::<crate::domain_types::ServiceSocketAddress>(constants_str::VALUE_127_0_0_1_3000)
            .expect(constants_str::DIAGNOSTIC_A8B92BAC);
    assert_eq!(
        address.get_inner().to_string(),
        constants_str::VALUE_127_0_0_1_3000
    );
    [0u16, u16::MAX].into_iter().fold((), |(), port| {
        [
            std::net::SocketAddr::from((std::net::Ipv4Addr::LOCALHOST, port)),
            std::net::SocketAddr::from((std::net::Ipv6Addr::LOCALHOST, port)),
        ]
        .into_iter()
        .fold((), |(), socket_addr| {
            assert!(
                parse_env::<crate::domain_types::ServiceSocketAddress>(&socket_addr.to_string())
                    .is_ok_and(|parsed| *parsed.get_inner() == socket_addr)
            );
        });
    });
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
    [constants_str::EMPTY.to_owned(), constants_str::VALUE_127_0_0_1.to_owned(), format!("{}:{}", constants_str::VALUE_127_0_0_1, u32::from(u16::MAX) + 1u32), format!("{}:{}", std::net::Ipv6Addr::LOCALHOST, 1u16)]
        .into_iter().fold((), |(), text| {
            let native = text.parse::<std::net::SocketAddr>();
            assert!(native.is_err());
            let Err(source) = native else { return; };
            assert!(parse_env::<crate::domain_types::ServiceSocketAddress>(&text).is_err_and(|socket_error| {
                let crate::domain_types::TryFromStdEnvVarOkServiceSocketAddressError::StdNetSocketAddr { std_net_socket_addr } = socket_error;
                format!("{std_net_socket_addr:?}") == format!("{source:?}")
            }));
        });
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
fn test_timezone_signed_boundaries_preserve_offsets_and_native_conversion() {
    [-86_399i32, -1i32, 0i32, 1i32, 86_399i32]
        .into_iter()
        .fold((), |(), seconds| {
            let direct = crate::chrono_timezone::ChronoTimezone::try_from(
                crate::timezone_seconds::TimezoneSeconds::from(seconds),
            );
            let native = chrono::FixedOffset::east_opt(seconds);
            assert!(direct.is_ok() && native.is_some());
            let (Ok(timezone), Some(fixed_offset)) = (direct, native) else {
                return;
            };
            assert_eq!(timezone.local_minus_utc(), seconds);
            assert_eq!(
                crate::chrono_timezone::ChronoTimezone::try_from(fixed_offset),
                Ok(timezone)
            );
            assert_eq!(
                crate::parse_east_fixed_offset::parse_east_fixed_offset(
                    crate::timezone_seconds::TimezoneSeconds::from(seconds)
                ),
                Ok(timezone)
            );
            assert!(
                parse_env::<crate::chrono_timezone::ChronoTimezone>(&seconds.to_string())
                    .is_ok_and(|parsed| parsed == timezone)
            );
        });
}

#[test]
fn test_timezone_rejects_both_exclusive_limits_with_exact_offset_error() {
    [-86_400i32, 86_400i32, i32::MIN, i32::MAX].into_iter().fold((), |(), seconds| {
        let expected = crate::chrono_fixed_offset_error::ChronoFixedOffsetError::from(constants_str::CONFIG_TIMEZONE_NOT_EAST_MSG);
        assert_eq!(crate::chrono_timezone::ChronoTimezone::try_from(crate::timezone_seconds::TimezoneSeconds::from(seconds)), Err(expected));
        assert_eq!(crate::parse_east_fixed_offset::parse_east_fixed_offset(crate::timezone_seconds::TimezoneSeconds::from(seconds)), Err(expected));
        assert!(parse_env::<crate::chrono_timezone::ChronoTimezone>(&seconds.to_string()).is_err_and(|error| matches!(error, crate::try_from_std_env_var_ok_timezone_error::TryFromStdEnvVarOkTimezoneError::ChronoFixedOffset { chrono_fixed_offset } if chrono_fixed_offset == expected)));
    });
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

#[test]
fn test_integer_configuration_parsers_preserve_native_invalid_input_diagnostics() {
    let mut positive_overflow = u64::MAX.to_string();
    positive_overflow.push('0');
    let mut negative_overflow = i64::MIN.to_string();
    negative_overflow.push('0');
    [
        constants_str::EMPTY.to_owned(),
        constants_str::X.to_owned(),
        [' ', '1'].into_iter().collect::<String>(),
        positive_overflow,
        negative_overflow,
    ]
    .into_iter()
    .fold((), |(), text| {
        assert!(text.parse::<i32>().is_err_and(|native| {
            parse_env::<crate::chrono_timezone::ChronoTimezone>(&text).is_err_and(|error| {
                let crate::try_from_std_env_var_ok_timezone_error::TryFromStdEnvVarOkTimezoneError::I32Parsing { i32_parsing } = error else { return false; };
                format!("{i32_parsing:?}") == format!("{native:?}")
            })
        }));
        assert!(text.parse::<usize>().is_err_and(|native| {
            parse_env::<crate::maximum_size_of_http_body_in_bytes::MaximumSizeOfHttpBodyInBytes>(&text).is_err_and(|error| {
                let crate::try_from_std_env_var_ok_maximum_size_of_http_body_in_bytes_error::TryFromStdEnvVarOkMaximumSizeOfHttpBodyInBytesError::UsizeParsing { usize_parsing } = error else { return false; };
                format!("{usize_parsing:?}") == format!("{native:?}")
            })
        }));
        assert!(text.parse::<u64>().is_err_and(|native| {
            parse_env::<crate::admin_access_token_ttl_seconds::AdminAccessTokenTtlSeconds>(&text).is_err_and(|error| {
                let crate::try_from_std_env_var_ok_admin_positive_u64_error::TryFromStdEnvVarOkAdminPositiveU64Error::Parse { admin_positive_u64_parsing } = error else { return false; };
                format!("{admin_positive_u64_parsing:?}") == format!("{native:?}")
            })
        }));
    });
}
