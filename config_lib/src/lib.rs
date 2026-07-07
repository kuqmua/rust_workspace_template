pub mod str_from_enum_macros;
pub mod types;
pub use gen_getter_traits_for_struct_fields::GenGetterTraitsForStructFields;
pub use try_from_env::TryFromEnv;
const ENV_VALUE_IS_EMPTY_MSG: &str = "is empty";
const TIMEZONE_NOT_EAST_MSG: &str = "not east";
pub trait TryFromStdEnvVarOk: Sized {
    type Error;
    fn try_from_std_env_var_ok(v: String) -> Result<Self, Self::Error>;
}
config_lib_macros::impl_try_from_non_empty_string!(
    CorsAllowOrigin,
    TryFromStdEnvVarOkCorsAllowOriginEr
);
config_lib_macros::impl_try_from_secret_url!(DatabaseUrl, TryFromStdEnvVarOkDatabaseUrlEr);
config_lib_macros::impl_try_from_parse!(
    EnableApiGitCommitCheck,
    TryFromStdEnvVarOkEnableApiGitCommitCheckEr,
    bool,
    BoolParsing,
    bool_parsing,
    std::str::ParseBoolError,
    Clone,
    Copy
);
config_lib_macros::impl_try_from_parse!(
    MaximumSizeOfHttpBodyInBytes,
    TryFromStdEnvVarOkMaximumSizeOfHttpBodyInBytesEr,
    usize,
    UsizeParsing,
    usize_parsing,
    std::num::ParseIntError,
    Clone,
    Copy
);
config_lib_macros::impl_try_from_secret_url!(MongoUrl, TryFromStdEnvVarOkMongoUrlEr);
config_lib_macros::impl_try_from_parse!(
    PgPoolMaxConnections,
    TryFromStdEnvVarOkPgPoolMaxConnectionsEr,
    u32,
    U32Parsing,
    u32_parsing,
    std::num::ParseIntError,
    Clone,
    Copy
);
config_lib_macros::impl_try_from_secret_url!(RedisUrl, TryFromStdEnvVarOkRedisUrlEr);
config_lib_macros::impl_try_from_parse!(
    ServiceSocketAddress,
    TryFromStdEnvVarOkServiceSocketAddressEr,
    std::net::SocketAddr,
    StdNetSocketAddr,
    std_net_socket_addr,
    std::net::AddrParseError,
    Clone,
    Copy
);
config_lib_macros::impl_try_from_parse_string_er!(
    SrcPlaceType,
    TryFromStdEnvVarOkSrcPlaceTypeEr,
    types::SrcPlaceType,
    AppStateSrcPlaceTypeParsing,
    app_state_src_place_type_parsing
);
config_lib_macros::impl_try_from_non_empty_string!(
    StartingCheckLink,
    TryFromStdEnvVarOkStartingCheckLinkEr
);
#[derive(Debug, Clone, Copy, gen_getter_traits_for_struct_fields::GenGetterTrait, optml::Optml)]
pub struct Timezone(pub chrono::FixedOffset);
#[derive(Debug, thiserror::Error, optml::Optml)]
pub enum TryFromStdEnvVarOkTimezoneEr {
    #[error("{chrono_fixed_offset:?}")]
    ChronoFixedOffset { chrono_fixed_offset: &'static str },
    #[error("{i32_parsing:?}")]
    I32Parsing {
        i32_parsing: std::num::ParseIntError,
    },
}
impl TryFromStdEnvVarOk for Timezone {
    type Error = TryFromStdEnvVarOkTimezoneEr;
    fn try_from_std_env_var_ok(v: String) -> Result<Self, Self::Error> {
        let i32_v =
            parse_from_str_with_er(&v, |i32_parsing| Self::Error::I32Parsing { i32_parsing })?;
        parse_east_fixed_offset(i32_v)
            .map_err(|chrono_fixed_offset| Self::Error::ChronoFixedOffset {
                chrono_fixed_offset,
            })
            .map(Self)
    }
}
config_lib_macros::impl_try_from_parse_string_er!(
    TracingLevel,
    TryFromStdEnvVarOkTracingLevelEr,
    types::TracingLevel,
    AppStateTracingLevelParsing,
    app_state_tracing_type_parsing
);
#[allow(clippy::single_call_fn)] // shared helper centralizes env var read + parse + error mapping for TryFromEnv derive output
pub fn parse_required_env_var<T, ParseEr, Er, MapEnvVarEr, Parse, MapParseEr>(
    env_var_name: &'static str,
    map_env_var_er: MapEnvVarEr,
    parse: Parse,
    map_parse_er: MapParseEr,
) -> Result<T, Er>
where
    MapEnvVarEr: FnOnce(std::env::VarError, String) -> Er,
    Parse: FnOnce(String) -> Result<T, ParseEr>,
    MapParseEr: FnOnce(ParseEr) -> Er,
{
    let v = std::env::var(env_var_name)
        .map_err(|std_env_var_er| map_env_var_er(std_env_var_er, env_var_name.to_owned()))?;
    parse(v).map_err(map_parse_er)
}
fn try_map_non_empty_env_value<T, Er>(
    v: String,
    mk_er: impl FnOnce(&'static str) -> Er,
    map_ok: impl FnOnce(String) -> T,
) -> Result<T, Er> {
    if v.is_empty() {
        return Err(mk_er(ENV_VALUE_IS_EMPTY_MSG));
    }
    Ok(map_ok(v))
}
fn parse_from_str_with_er<T, ParseEr, Er>(
    v: &str,
    mk_er: impl FnOnce(ParseEr) -> Er,
) -> Result<T, Er>
where
    T: std::str::FromStr<Err = ParseEr>,
{
    v.parse::<T>().map_err(mk_er)
}
#[allow(clippy::single_call_fn)] // extracted timezone conversion keeps conversion + message mapping reusable and directly testable
fn parse_east_fixed_offset(v: i32) -> Result<chrono::FixedOffset, &'static str> {
    chrono::FixedOffset::east_opt(v).ok_or(TIMEZONE_NOT_EAST_MSG)
}
#[cfg(test)]
mod tests {
    #[derive(Debug, PartialEq, Eq)]
    enum ParseRequiredEnvVarTestEr {
        EnvVar { env_var_name: String },
        Parse { parse: &'static str },
    }
    fn parse_env<T>(v: &str) -> Result<T, T::Error>
    where
        T: super::TryFromStdEnvVarOk,
    {
        T::try_from_std_env_var_ok(v.to_owned())
    }
    #[test]
    fn cors_allow_origin_parsing_returns_value() {
        config_lib_macros::assert_parse_ok_matches!(
            super::CorsAllowOrigin,
            "*",
            super::CorsAllowOrigin(_)
        );
    }
    #[test]
    fn cors_allow_origin_parsing_returns_error_for_empty_string() {
        config_lib_macros::assert_empty_parse_err_matches!(
            super::CorsAllowOrigin,
            super::TryFromStdEnvVarOkCorsAllowOriginEr::IsEmpty { .. }
        );
    }
    #[test]
    fn database_url_parsing_returns_value_for_non_empty_input() {
        config_lib_macros::assert_parse_ok_matches!(
            super::DatabaseUrl,
            "postgres://db",
            super::DatabaseUrl(_)
        );
    }
    #[test]
    fn database_url_parsing_returns_error_for_empty_string() {
        config_lib_macros::assert_empty_parse_err_matches!(
            super::DatabaseUrl,
            super::TryFromStdEnvVarOkDatabaseUrlEr::IsEmpty { .. }
        );
    }
    #[test]
    fn mongo_url_parsing_returns_value_for_non_empty_input() {
        config_lib_macros::assert_parse_ok_matches!(
            super::MongoUrl,
            "mongodb://db",
            super::MongoUrl(_)
        );
    }
    #[test]
    fn mongo_url_parsing_returns_error_for_empty_string() {
        config_lib_macros::assert_empty_parse_err_matches!(
            super::MongoUrl,
            super::TryFromStdEnvVarOkMongoUrlEr::IsEmpty { .. }
        );
    }
    #[test]
    fn redis_url_parsing_returns_value_for_non_empty_input() {
        config_lib_macros::assert_parse_ok_matches!(
            super::RedisUrl,
            "redis://db",
            super::RedisUrl(_)
        );
    }
    #[test]
    fn redis_url_parsing_returns_error_for_empty_string() {
        config_lib_macros::assert_empty_parse_err_matches!(
            super::RedisUrl,
            super::TryFromStdEnvVarOkRedisUrlEr::IsEmpty { .. }
        );
    }
    #[test]
    fn src_place_type_parsing_is_case_insensitive() {
        config_lib_macros::assert_parse_ok_matches!(
            super::SrcPlaceType,
            "GITHUB",
            super::SrcPlaceType(super::types::SrcPlaceType::Github)
        );
    }
    #[test]
    fn src_place_type_parsing_returns_error_for_unknown_value() {
        config_lib_macros::assert_parse_err_matches!(
            super::SrcPlaceType,
            "bad",
            super::TryFromStdEnvVarOkSrcPlaceTypeEr::AppStateSrcPlaceTypeParsing { .. }
        );
    }
    #[test]
    fn tracing_level_parsing_is_case_insensitive() {
        config_lib_macros::assert_parse_ok_matches!(
            super::TracingLevel,
            "DeBuG",
            super::TracingLevel(super::types::TracingLevel::Debug)
        );
    }
    #[test]
    fn tracing_level_parsing_returns_error_for_unknown_value() {
        config_lib_macros::assert_parse_err_matches!(
            super::TracingLevel,
            "bad",
            super::TryFromStdEnvVarOkTracingLevelEr::AppStateTracingLevelParsing { .. }
        );
    }
    #[test]
    fn enable_api_git_commit_check_parsing_returns_bool() {
        config_lib_macros::assert_parse_ok_matches!(
            super::EnableApiGitCommitCheck,
            "true",
            super::EnableApiGitCommitCheck(true)
        );
    }
    #[test]
    fn enable_api_git_commit_check_parsing_returns_error_for_invalid_bool() {
        config_lib_macros::assert_parse_err_matches!(
            super::EnableApiGitCommitCheck,
            "truthy",
            super::TryFromStdEnvVarOkEnableApiGitCommitCheckEr::BoolParsing { .. }
        );
    }
    #[test]
    fn maximum_size_of_http_body_in_bytes_parsing_returns_usize() {
        config_lib_macros::assert_parse_ok_matches!(
            super::MaximumSizeOfHttpBodyInBytes,
            "128",
            super::MaximumSizeOfHttpBodyInBytes(128)
        );
    }
    #[test]
    fn maximum_size_of_http_body_in_bytes_parsing_returns_error_for_invalid_number() {
        config_lib_macros::assert_parse_err_matches!(
            super::MaximumSizeOfHttpBodyInBytes,
            "1k",
            super::TryFromStdEnvVarOkMaximumSizeOfHttpBodyInBytesEr::UsizeParsing { .. }
        );
    }
    #[test]
    fn pg_pool_max_connections_parsing_returns_u32() {
        config_lib_macros::assert_parse_ok_matches!(
            super::PgPoolMaxConnections,
            "10",
            super::PgPoolMaxConnections(10)
        );
    }
    #[test]
    fn pg_pool_max_connections_parsing_returns_error_for_invalid_number() {
        config_lib_macros::assert_parse_err_matches!(
            super::PgPoolMaxConnections,
            "bad",
            super::TryFromStdEnvVarOkPgPoolMaxConnectionsEr::U32Parsing { .. }
        );
    }
    #[test]
    fn non_empty_string_parser_returns_error_for_empty_value() {
        config_lib_macros::assert_empty_parse_err_matches!(
            super::StartingCheckLink,
            super::TryFromStdEnvVarOkStartingCheckLinkEr::IsEmpty { .. }
        );
    }
    #[test]
    fn non_empty_string_parser_returns_value_for_non_empty_value() {
        config_lib_macros::assert_parse_ok_matches!(
            super::StartingCheckLink,
            "https://example.com",
            super::StartingCheckLink(_)
        );
    }
    #[test]
    fn service_socket_address_parsing_returns_socket_addr() {
        config_lib_macros::assert_parse_ok_matches!(
            super::ServiceSocketAddress,
            "127.0.0.1:3000",
            super::ServiceSocketAddress(_)
        );
    }
    #[test]
    fn service_socket_address_parsing_returns_error_for_invalid_addr() {
        let er = parse_env::<super::ServiceSocketAddress>("127.0.0.1");
        assert!(matches!(
            er,
            Err(super::TryFromStdEnvVarOkServiceSocketAddressEr::StdNetSocketAddr { .. })
        ));
    }
    #[test]
    fn timezone_parsing_returns_timezone_for_valid_offset() {
        config_lib_macros::assert_parse_ok_matches!(super::Timezone, "0", super::Timezone(_));
    }
    #[test]
    fn timezone_parsing_returns_i32_error_for_non_number() {
        config_lib_macros::assert_parse_err_matches!(
            super::Timezone,
            "nan",
            super::TryFromStdEnvVarOkTimezoneEr::I32Parsing { .. }
        );
    }
    #[test]
    fn parse_east_fixed_offset_returns_offset_for_valid_seconds() {
        let parsed = super::parse_east_fixed_offset(3i32 * 3_600i32);
        assert!(matches!(parsed, Ok(v) if v.local_minus_utc() == 3i32 * 3_600i32));
    }
    #[test]
    fn parse_east_fixed_offset_returns_error_for_out_of_range_seconds() {
        let parsed = super::parse_east_fixed_offset(i32::MAX);
        assert_eq!(parsed, Err(super::TIMEZONE_NOT_EAST_MSG));
    }
    #[test]
    fn timezone_parsing_returns_offset_error_when_out_of_range() {
        let out_of_range = i32::MAX.to_string();
        let er = parse_env::<super::Timezone>(&out_of_range);
        assert!(matches!(
            er,
            Err(super::TryFromStdEnvVarOkTimezoneEr::ChronoFixedOffset { .. })
        ));
    }
    #[test]
    fn parse_required_env_var_parses_value_when_env_var_exists() {
        let parsed = super::parse_required_env_var(
            "PATH",
            |_std_env_var_er, env_var_name| ParseRequiredEnvVarTestEr::EnvVar { env_var_name },
            |v| Ok::<_, &'static str>(v.len()),
            |parse| ParseRequiredEnvVarTestEr::Parse { parse },
        );
        assert!(matches!(parsed, Ok(v) if v > 0));
    }
    #[test]
    fn parse_required_env_var_maps_missing_env_var_error() {
        let parsed = super::parse_required_env_var(
            "CONFIG_LIB_TEST_ENV_VAR_4E8A7F21",
            |_std_env_var_er, env_var_name| ParseRequiredEnvVarTestEr::EnvVar { env_var_name },
            Ok::<_, &'static str>,
            |parse| ParseRequiredEnvVarTestEr::Parse { parse },
        );
        assert_eq!(
            parsed,
            Err(ParseRequiredEnvVarTestEr::EnvVar {
                env_var_name: "CONFIG_LIB_TEST_ENV_VAR_4E8A7F21".to_owned()
            })
        );
    }
    #[test]
    fn parse_required_env_var_maps_parse_error() {
        let parsed = super::parse_required_env_var(
            "PATH",
            |_std_env_var_er, env_var_name| ParseRequiredEnvVarTestEr::EnvVar { env_var_name },
            |_v| Err::<(), _>("parse failed"),
            |parse| ParseRequiredEnvVarTestEr::Parse { parse },
        );
        assert_eq!(
            parsed,
            Err(ParseRequiredEnvVarTestEr::Parse {
                parse: "parse failed"
            })
        );
    }
}
