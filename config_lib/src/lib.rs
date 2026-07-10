pub mod types;
const ENV_VALUE_IS_EMPTY_MSG: &str = "is empty";
const TIMEZONE_NOT_EAST_MSG: &str = "not east";
const CONFIG_LIB_STRING_WRAPPER_MAX_LEN: usize = 1_048_576;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StdEnvVarOk(String);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigLibStringWrapperTryFromStringEr {
    TooLong { len: usize, max: usize },
}
impl std::fmt::Display for ConfigLibStringWrapperTryFromStringEr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooLong { len, max } => {
                write!(
                    f,
                    "config string wrapper length {len} exceeds maximum {max}"
                )
            }
        }
    }
}
impl From<ConfigLibStringWrapperTryFromStringEr> for StdEnvVarOk {
    fn from(value: ConfigLibStringWrapperTryFromStringEr) -> Self {
        Self(value.to_string())
    }
}
impl TryFrom<String> for StdEnvVarOk {
    type Error = ConfigLibStringWrapperTryFromStringEr;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > CONFIG_LIB_STRING_WRAPPER_MAX_LEN {
            return Err(Self::Error::TooLong {
                len: value.len(),
                max: CONFIG_LIB_STRING_WRAPPER_MAX_LEN,
            });
        }
        Ok(Self(value))
    }
}
#[derive(Debug, Clone, Copy, newtype::Newtype)]
#[newtype(from_inner)]
pub struct StdEnvVarOkRef<'value_lt>(&'value_lt str);
#[derive(Debug, Clone, Copy, newtype::Newtype)]
#[newtype(from_inner)]
pub struct EnvVarNameRef<'name_lt>(&'name_lt str);
#[derive(Debug, Clone, PartialEq, Eq, newtype::Newtype)]
#[newtype(display)]
pub struct EnvVarName(String);
impl From<ConfigLibStringWrapperTryFromStringEr> for EnvVarName {
    fn from(value: ConfigLibStringWrapperTryFromStringEr) -> Self {
        Self(value.to_string())
    }
}
impl TryFrom<String> for EnvVarName {
    type Error = ConfigLibStringWrapperTryFromStringEr;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > CONFIG_LIB_STRING_WRAPPER_MAX_LEN {
            return Err(Self::Error::TooLong {
                len: value.len(),
                max: CONFIG_LIB_STRING_WRAPPER_MAX_LEN,
            });
        }
        Ok(Self(value))
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChronoFixedOffsetEr(&'static str);
#[derive(newtype::Newtype)]
#[newtype(debug_transparent)]
pub struct StdI32ParsingEr(std::num::ParseIntError);
#[derive(newtype::Newtype)]
#[newtype(debug_transparent)]
pub struct StdU32ParsingEr(std::num::ParseIntError);
#[derive(newtype::Newtype)]
#[newtype(debug_transparent)]
pub struct StdUsizeParsingEr(std::num::ParseIntError);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TimezoneSeconds(i32);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ChronoEastFixedOffset(chrono::FixedOffset);
pub trait TryFromStdEnvVarOk: Sized {
    type Error;
    fn try_from_std_env_var_ok(v: StdEnvVarOk) -> Result<Self, Self::Error>;
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
#[derive(Debug, Clone, Copy, gen_getter_traits_for_struct_fields::GenGetterTrait, optml::Optml)]
pub struct MaximumSizeOfHttpBodyInBytes(usize);
impl std::ops::Deref for MaximumSizeOfHttpBodyInBytes {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error, optml::Optml)]
pub enum MaximumSizeOfHttpBodyInBytesTryFromUsizeEr {
    #[error("maximum size of http body in bytes must be greater than zero")]
    IsZero,
}
impl TryFrom<usize> for MaximumSizeOfHttpBodyInBytes {
    type Error = MaximumSizeOfHttpBodyInBytesTryFromUsizeEr;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(Self::Error::IsZero)
        } else {
            Ok(Self(value))
        }
    }
}
#[derive(Debug, thiserror::Error, optml::Optml)]
pub enum TryFromStdEnvVarOkMaximumSizeOfHttpBodyInBytesEr {
    #[error("{maximum_size_of_http_body_in_bytes:?}")]
    MaximumSizeOfHttpBodyInBytes {
        maximum_size_of_http_body_in_bytes: MaximumSizeOfHttpBodyInBytesTryFromUsizeEr,
    },
    #[error("{:?}", .usize_parsing)]
    UsizeParsing { usize_parsing: StdUsizeParsingEr },
}
impl TryFromStdEnvVarOk for MaximumSizeOfHttpBodyInBytes {
    type Error = TryFromStdEnvVarOkMaximumSizeOfHttpBodyInBytesEr;
    fn try_from_std_env_var_ok(v: StdEnvVarOk) -> Result<Self, Self::Error> {
        let parsed: usize =
            parse_from_str_with_er(StdEnvVarOkRef(v.0.as_str()), |usize_parsing| {
                Self::Error::UsizeParsing {
                    usize_parsing: StdUsizeParsingEr(usize_parsing),
                }
            })?;
        Self::try_from(parsed).map_err(|maximum_size_of_http_body_in_bytes| {
            Self::Error::MaximumSizeOfHttpBodyInBytes {
                maximum_size_of_http_body_in_bytes,
            }
        })
    }
}
config_lib_macros::impl_try_from_secret_url!(MongoUrl, TryFromStdEnvVarOkMongoUrlEr);
#[derive(Debug, Clone, Copy, gen_getter_traits_for_struct_fields::GenGetterTrait, optml::Optml)]
pub struct PgPoolMaxConnections(u32);
impl std::ops::Deref for PgPoolMaxConnections {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error, optml::Optml)]
pub enum PgPoolMaxConnectionsTryFromU32Er {
    #[error("pg pool max connections must be greater than zero")]
    IsZero,
}
impl TryFrom<u32> for PgPoolMaxConnections {
    type Error = PgPoolMaxConnectionsTryFromU32Er;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(Self::Error::IsZero)
        } else {
            Ok(Self(value))
        }
    }
}
#[derive(Debug, thiserror::Error, optml::Optml)]
pub enum TryFromStdEnvVarOkPgPoolMaxConnectionsEr {
    #[error("{pg_pool_max_connections:?}")]
    PgPoolMaxConnections {
        pg_pool_max_connections: PgPoolMaxConnectionsTryFromU32Er,
    },
    #[error("{:?}", .u32_parsing)]
    U32Parsing { u32_parsing: StdU32ParsingEr },
}
impl TryFromStdEnvVarOk for PgPoolMaxConnections {
    type Error = TryFromStdEnvVarOkPgPoolMaxConnectionsEr;
    fn try_from_std_env_var_ok(v: StdEnvVarOk) -> Result<Self, Self::Error> {
        let parsed: u32 = parse_from_str_with_er(StdEnvVarOkRef(v.0.as_str()), |u32_parsing| {
            Self::Error::U32Parsing {
                u32_parsing: StdU32ParsingEr(u32_parsing),
            }
        })?;
        Self::try_from(parsed).map_err(|pg_pool_max_connections| {
            Self::Error::PgPoolMaxConnections {
                pg_pool_max_connections,
            }
        })
    }
}
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
pub struct ChronoTimezone(chrono::FixedOffset);
impl std::ops::Deref for ChronoTimezone {
    type Target = chrono::FixedOffset;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl TryFrom<chrono::FixedOffset> for ChronoTimezone {
    type Error = ChronoFixedOffsetEr;
    fn try_from(value: chrono::FixedOffset) -> Result<Self, Self::Error> {
        parse_east_fixed_offset(TimezoneSeconds(value.local_minus_utc())).map(|_| Self(value))
    }
}
#[derive(Debug, thiserror::Error, optml::Optml)]
pub enum TryFromStdEnvVarOkTimezoneEr {
    #[error("{chrono_fixed_offset:?}")]
    ChronoFixedOffset {
        chrono_fixed_offset: ChronoFixedOffsetEr,
    },
    #[error("{i32_parsing:?}")]
    I32Parsing { i32_parsing: StdI32ParsingEr },
}
impl TryFromStdEnvVarOk for ChronoTimezone {
    type Error = TryFromStdEnvVarOkTimezoneEr;
    fn try_from_std_env_var_ok(v: StdEnvVarOk) -> Result<Self, Self::Error> {
        let i32_v = TimezoneSeconds(parse_from_str_with_er(
            StdEnvVarOkRef(v.0.as_str()),
            |i32_parsing| Self::Error::I32Parsing {
                i32_parsing: StdI32ParsingEr(i32_parsing),
            },
        )?);
        parse_east_fixed_offset(i32_v)
            .map_err(|chrono_fixed_offset| Self::Error::ChronoFixedOffset {
                chrono_fixed_offset,
            })
            .map(|timezone| Self(timezone.0))
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
    env_var_name: EnvVarNameRef<'_>,
    map_env_var_er: MapEnvVarEr,
    parse: Parse,
    map_parse_er: MapParseEr,
) -> Result<T, Er>
where
    MapEnvVarEr: FnOnce(std::env::VarError, EnvVarName) -> Er,
    Parse: FnOnce(StdEnvVarOk) -> Result<T, ParseEr>,
    MapParseEr: FnOnce(ParseEr) -> Er,
{
    let v = std::env::var(env_var_name.0).map_err(|std_env_var_er| {
        map_env_var_er(
            std_env_var_er,
            EnvVarName::try_from(env_var_name.0.to_owned()).unwrap_or_else(EnvVarName::from),
        )
    })?;
    parse(StdEnvVarOk::try_from(v).unwrap_or_else(StdEnvVarOk::from)).map_err(map_parse_er)
}
fn try_map_non_empty_env_value<T, Er>(
    v: StdEnvVarOk,
    mk_er: impl FnOnce(&'static str) -> Er,
    map_ok: impl FnOnce(String) -> T,
) -> Result<T, Er> {
    if v.0.is_empty() {
        return Err(mk_er(ENV_VALUE_IS_EMPTY_MSG));
    }
    Ok(map_ok(v.0))
}
fn parse_from_str_with_er<T, ParseEr, Er>(
    v: StdEnvVarOkRef<'_>,
    mk_er: impl FnOnce(ParseEr) -> Er,
) -> Result<T, Er>
where
    T: std::str::FromStr<Err = ParseEr>,
{
    v.0.parse::<T>().map_err(mk_er)
}
#[allow(clippy::single_call_fn)] // extracted timezone conversion keeps conversion + message mapping reusable and directly testable
fn parse_east_fixed_offset(
    v: TimezoneSeconds,
) -> Result<ChronoEastFixedOffset, ChronoFixedOffsetEr> {
    chrono::FixedOffset::east_opt(v.0)
        .map(ChronoEastFixedOffset)
        .ok_or(ChronoFixedOffsetEr(TIMEZONE_NOT_EAST_MSG))
}
#[cfg(test)]
mod tests {
    #[derive(Debug, PartialEq, Eq)]
    enum ParseRequiredEnvVarTestEr {
        EnvVar { env_var_name: super::EnvVarName },
        Parse { parse: &'static str },
    }
    fn parse_env<T>(v: &str) -> Result<T, T::Error>
    where
        T: super::TryFromStdEnvVarOk,
    {
        T::try_from_std_env_var_ok(
            super::StdEnvVarOk::try_from(v.to_owned()).unwrap_or_else(super::StdEnvVarOk::from),
        )
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
    fn maximum_size_of_http_body_in_bytes_parsing_returns_error_for_zero() {
        config_lib_macros::assert_parse_err_matches!(
            super::MaximumSizeOfHttpBodyInBytes,
            "0",
            super::TryFromStdEnvVarOkMaximumSizeOfHttpBodyInBytesEr::MaximumSizeOfHttpBodyInBytes { .. }
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
    fn pg_pool_max_connections_parsing_returns_error_for_zero() {
        config_lib_macros::assert_parse_err_matches!(
            super::PgPoolMaxConnections,
            "0",
            super::TryFromStdEnvVarOkPgPoolMaxConnectionsEr::PgPoolMaxConnections { .. }
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
        config_lib_macros::assert_parse_ok_matches!(
            super::ChronoTimezone,
            "0",
            super::ChronoTimezone(_)
        );
    }
    #[test]
    fn timezone_parsing_returns_i32_error_for_non_number() {
        config_lib_macros::assert_parse_err_matches!(
            super::ChronoTimezone,
            "nan",
            super::TryFromStdEnvVarOkTimezoneEr::I32Parsing { .. }
        );
    }
    #[test]
    fn parse_east_fixed_offset_returns_offset_for_valid_seconds() {
        let parsed = super::parse_east_fixed_offset(super::TimezoneSeconds(3i32 * 3_600i32));
        assert!(matches!(parsed, Ok(v) if v.0.local_minus_utc() == 3i32 * 3_600i32));
    }
    #[test]
    fn parse_east_fixed_offset_returns_error_for_out_of_range_seconds() {
        let parsed = super::parse_east_fixed_offset(super::TimezoneSeconds(i32::MAX));
        assert_eq!(
            parsed,
            Err(super::ChronoFixedOffsetEr(super::TIMEZONE_NOT_EAST_MSG))
        );
    }
    #[test]
    fn timezone_parsing_returns_offset_error_when_out_of_range() {
        let out_of_range = i32::MAX.to_string();
        let er = parse_env::<super::ChronoTimezone>(&out_of_range);
        assert!(matches!(
            er,
            Err(super::TryFromStdEnvVarOkTimezoneEr::ChronoFixedOffset { .. })
        ));
    }
    #[test]
    fn parse_required_env_var_parses_value_when_env_var_exists() {
        let parsed = super::parse_required_env_var(
            super::EnvVarNameRef("PATH"),
            |_std_env_var_er, env_var_name| ParseRequiredEnvVarTestEr::EnvVar { env_var_name },
            |v| Ok::<_, &'static str>(v.0.len()),
            |parse| ParseRequiredEnvVarTestEr::Parse { parse },
        );
        assert!(matches!(parsed, Ok(v) if v > 0));
    }
    #[test]
    fn parse_required_env_var_maps_missing_env_var_error() {
        let parsed = super::parse_required_env_var(
            super::EnvVarNameRef("CONFIG_LIB_TEST_ENV_VAR_4E8A7F21"),
            |_std_env_var_er, env_var_name| ParseRequiredEnvVarTestEr::EnvVar { env_var_name },
            Ok::<_, &'static str>,
            |parse| ParseRequiredEnvVarTestEr::Parse { parse },
        );
        assert_eq!(
            parsed,
            Err(ParseRequiredEnvVarTestEr::EnvVar {
                env_var_name: super::EnvVarName::try_from(
                    "CONFIG_LIB_TEST_ENV_VAR_4E8A7F21".to_owned()
                )
                .unwrap_or_else(super::EnvVarName::from)
            })
        );
    }
    #[test]
    fn parse_required_env_var_maps_parse_error() {
        let parsed = super::parse_required_env_var(
            super::EnvVarNameRef("PATH"),
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
