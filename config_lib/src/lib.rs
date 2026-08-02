#![allow(clippy::arbitrary_source_item_ordering)] // configuration declarations stay grouped with their parse errors and TryFromStdEnvVarOk implementations
mod admin;
mod admin_jwt;
mod bool_flags;
mod http;
mod pg_pool;
pub mod types;
pub use admin::{
    AdminAccessTokenTtlSeconds, AdminLoginFailureLimit, AdminPasswordHashConcurrency,
    AdminPositiveU64ParsingError, AdminPositiveUsizeParsingError, AdminRefreshTokenTtlSeconds,
    AdminSessionLimit, AdminSignInRateLimit, AdminTokenAudience,
    AdminTokenAudienceTryFromStringError, AdminTokenIssuer, AdminTokenIssuerTryFromStringError,
    GetAdminAccessTokenTtlSeconds, GetAdminLoginFailureLimit, GetAdminPasswordHashConcurrency,
    GetAdminRefreshTokenTtlSeconds, GetAdminSessionLimit, GetAdminSignInRateLimit,
    GetAdminTokenAudience, GetAdminTokenIssuer,
    TryFromStdEnvVarOkAdminPasswordHashConcurrencyError, TryFromStdEnvVarOkAdminPositiveU64Error,
    TryFromStdEnvVarOkAdminTokenTextError,
};
pub use admin_jwt::{AdminJwtSecret, GetAdminJwtSecret, TryFromStdEnvVarOkAdminJwtSecretError};
pub use bool_flags::{
    AdminBoolParsingError, AdminCookieSecure, AdminSwaggerEnabled, GetAdminCookieSecure,
    GetAdminSwaggerEnabled, HttpGzipEnabled, ProductionMode,
    TryFromStdEnvVarOkAdminCookieSecureError,
};
pub use http::{
    ContentSecurityPolicy, ContentSecurityPolicyError, GetMaximumSizeOfHttpBodyInBytes,
    MaximumSizeOfHttpBodyInBytes, MaximumSizeOfHttpBodyInBytesTryFromUsizeError,
    TryFromStdEnvVarOkMaximumSizeOfHttpBodyInBytesError,
};
pub use pg_pool::{
    GetPgPoolMaxConnections, PgPoolAcquireTimeoutSeconds, PgPoolConfigParseError,
    PgPoolIdleTimeoutSeconds, PgPoolMaxConnections, PgPoolMaxConnectionsTryFromU32Error,
    PgPoolMaxLifetimeSeconds, PgPoolMinConnections, RequestTimeoutSeconds,
    TryFromStdEnvVarOkPgPoolMaxConnectionsError,
};
const CONFIG_LIB_STRING_WRAPPER_MAX_LEN: usize = 1_048_576;
#[derive(optml::Optml, Debug, Clone, PartialEq, Eq)]
pub struct StdEnvVarOk(String);
#[derive(optml::Optml, Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum ConfigLibStringWrapperTryFromStringError {
    #[error("config string wrapper length {len} exceeds maximum {max}")]
    TooLong { len: usize, max: usize },
}
impl From<ConfigLibStringWrapperTryFromStringError> for StdEnvVarOk {
    fn from(value: ConfigLibStringWrapperTryFromStringError) -> Self {
        Self(value.to_string())
    }
}
impl TryFrom<String> for StdEnvVarOk {
    type Error = ConfigLibStringWrapperTryFromStringError;
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
#[derive(optml::Optml, Debug, Clone, Copy, newtype::FromInner)]
pub struct StdEnvVarOkRef<'value_lt>(&'value_lt str);
#[derive(optml::Optml, Debug, Clone, Copy, newtype::AsRefInner, newtype::FromInner)]
pub struct EnvVarNameRef<'name_lt>(&'name_lt str);
#[derive(optml::Optml, Debug, Clone, PartialEq, Eq, newtype::Display)]
pub struct EnvVarName(String);
impl From<ConfigLibStringWrapperTryFromStringError> for EnvVarName {
    fn from(value: ConfigLibStringWrapperTryFromStringError) -> Self {
        Self(value.to_string())
    }
}
impl TryFrom<String> for EnvVarName {
    type Error = ConfigLibStringWrapperTryFromStringError;
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
#[derive(optml::Optml, Debug, Clone, Copy, PartialEq, Eq, newtype::FromInner)]
pub struct ChronoFixedOffsetError(&'static str);
#[derive(optml::Optml, newtype::DebugTransparent, newtype::FromInner)]
pub struct StdI32ParsingError(std::num::ParseIntError);
#[derive(optml::Optml, newtype::DebugTransparent, newtype::FromInner)]
pub struct StdU32ParsingError(std::num::ParseIntError);
#[derive(optml::Optml, newtype::DebugTransparent, newtype::FromInner)]
pub struct StdUsizeParsingError(std::num::ParseIntError);
#[derive(optml::Optml, Debug, Clone, Copy, PartialEq, Eq, newtype::FromInner)]
struct TimezoneSeconds(i32);
#[derive(optml::Optml, Debug, Clone, Copy, PartialEq, Eq, newtype::FromInner)]
struct ChronoEastFixedOffset(chrono::FixedOffset);
pub trait TryFromStdEnvVarOk: Sized {
    type Error;
    fn try_from_std_env_var_ok(v: StdEnvVarOk) -> Result<Self, Self::Error>;
}
#[derive(optml::Optml, Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConfigFieldSensitivity {
    Public,
    Secret,
}
#[derive(optml::Optml, Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConfigFieldRequirement {
    Required,
}
#[derive(optml::Optml, Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConfigExampleValidity {
    Invalid,
    Valid,
}
#[derive(optml::Optml, Clone, Copy, Debug, newtype::AsRefInner, newtype::FromInner)]
pub struct ConfigFieldExampleRef<'example_lt>(&'example_lt str);
#[derive(optml::Optml, Clone, Copy, Debug, newtype::AsRefInner, newtype::FromInner)]
pub struct ConfigRustTypeName(&'static str);
#[derive(optml::Optml, Clone, Copy)]
pub struct ConfigFieldDescriptor {
    env_name: EnvVarNameRef<'static>,
    example: ConfigFieldExampleRef<'static>,
    parser: fn(StdEnvVarOk) -> ConfigExampleValidity,
    rust_type_name: ConfigRustTypeName,
    requirement: ConfigFieldRequirement,
    sensitivity: ConfigFieldSensitivity,
}
impl std::fmt::Debug for ConfigFieldDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(str_constants::CONFIG_FIELD_DESCRIPTOR)
            .field(str_constants::ENV_NAME, &self.env_name)
            .field(str_constants::EXAMPLE, &self.example)
            .field(str_constants::REQUIRED, &self.requirement)
            .field(str_constants::RUST_TYPE_NAME, &self.rust_type_name)
            .field(str_constants::SENSITIVITY, &self.sensitivity)
            .finish_non_exhaustive()
    }
}
impl ConfigFieldDescriptor {
    #[must_use]
    pub const fn new(
        env_name: EnvVarNameRef<'static>,
        example: ConfigFieldExampleRef<'static>,
        parser: fn(StdEnvVarOk) -> ConfigExampleValidity,
        requirement: ConfigFieldRequirement,
        rust_type_name: ConfigRustTypeName,
        sensitivity: ConfigFieldSensitivity,
    ) -> Self {
        Self {
            env_name,
            example,
            parser,
            rust_type_name,
            requirement,
            sensitivity,
        }
    }
    #[must_use]
    pub const fn env_name(self) -> EnvVarNameRef<'static> {
        self.env_name
    }
    #[must_use]
    pub const fn example(self) -> ConfigFieldExampleRef<'static> {
        self.example
    }
    #[must_use]
    pub const fn requirement(self) -> ConfigFieldRequirement {
        self.requirement
    }
    #[must_use]
    pub const fn rust_type_name(self) -> ConfigRustTypeName {
        self.rust_type_name
    }
    #[must_use]
    pub const fn sensitivity(self) -> ConfigFieldSensitivity {
        self.sensitivity
    }
    #[must_use]
    pub fn validate_example(self, value: StdEnvVarOk) -> ConfigExampleValidity {
        (self.parser)(value)
    }
}
#[derive(
    optml::Optml,
    Clone,
    PartialEq,
    Eq,
    newtype::AsRefOwned,
    newtype::BoundedString,
    newtype::DebugRedacted,
    newtype::DerefInner,
)]
#[bounded_string(max = 1_048_576, description = "configuration secret text")]
pub struct StdConfigSecretString(String);
impl secrecy::zeroize::Zeroize for StdConfigSecretString {
    fn zeroize(&mut self) {
        secrecy::zeroize::Zeroize::zeroize(&mut self.0);
    }
}
#[derive(optml::Optml, newtype::AsRefOwned, newtype::FromInner)]
pub struct SecrecySecretBoxString(secrecy::SecretBox<StdConfigSecretString>);
impl TryFrom<String> for SecrecySecretBoxString {
    type Error = StdConfigSecretStringTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        StdConfigSecretString::try_from(value)
            .map(|bounded| Self::from(secrecy::SecretBox::new(Box::new(bounded))))
    }
}
impl std::fmt::Debug for SecrecySecretBoxString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(str_constants::REDACTED_ALT_3)
    }
}
#[derive(
    optml::Optml, Debug, Clone, Copy, PartialEq, Eq, newtype::DerefInner, newtype::FromInner,
)]
pub struct StdNonZeroU64(std::num::NonZeroU64);
#[derive(
    optml::Optml, Debug, Clone, Copy, PartialEq, Eq, newtype::DerefInner, newtype::FromInner,
)]
pub struct StdNonZeroUsize(std::num::NonZeroUsize);
#[derive(optml::Optml, newtype::DebugTransparent, newtype::FromInner)]
pub struct StdParseIntError(std::num::ParseIntError);
#[derive(optml::Optml, newtype::DebugTransparent, newtype::FromInner)]
pub struct StdParseBoolError(std::str::ParseBoolError);
config_lib_macros::impl_try_from_non_empty_string!(
    CorsAllowOrigin,
    TryFromStdEnvVarOkCorsAllowOriginError
);
config_lib_macros::impl_try_from_non_empty_string!(
    TrustedProxyRangesText,
    TryFromStdEnvVarOkTrustedProxyRangesTextError
);
config_lib_macros::impl_try_from_secret_url!(DatabaseUrl, TryFromStdEnvVarOkDatabaseUrlError);
config_lib_macros::impl_try_from_parse!(
    EnableApiGitCommitCheck,
    TryFromStdEnvVarOkEnableApiGitCommitCheckError,
    bool,
    BoolParsing,
    bool_parsing,
    std::str::ParseBoolError,
    Clone,
    Copy
);
config_lib_macros::impl_try_from_secret_url!(MongoUrl, TryFromStdEnvVarOkMongoUrlError);

config_lib_macros::impl_try_from_secret_url!(RedisUrl, TryFromStdEnvVarOkRedisUrlError);
config_lib_macros::impl_try_from_parse!(
    ServiceSocketAddress,
    TryFromStdEnvVarOkServiceSocketAddressError,
    std::net::SocketAddr,
    StdNetSocketAddr,
    std_net_socket_addr,
    std::net::AddrParseError,
    Clone,
    Copy
);
config_lib_macros::impl_try_from_parse_string_error!(
    SrcPlaceType,
    TryFromStdEnvVarOkSrcPlaceTypeError,
    types::SrcPlaceType,
    AppStateSrcPlaceTypeParsing,
    app_state_src_place_type_parsing
);
config_lib_macros::impl_try_from_non_empty_string!(
    StartingCheckLink,
    TryFromStdEnvVarOkStartingCheckLinkError
);
#[derive(
    Debug,
    Clone,
    Copy,
    generate_getter_traits_for_struct_fields::GenerateGetterTrait,
    optml::Optml,
    newtype::DerefInner,
)]
pub struct ChronoTimezone(chrono::FixedOffset);
impl From<ChronoEastFixedOffset> for ChronoTimezone {
    fn from(value: ChronoEastFixedOffset) -> Self {
        Self(value.0)
    }
}
impl TryFrom<chrono::FixedOffset> for ChronoTimezone {
    type Error = ChronoFixedOffsetError;
    fn try_from(value: chrono::FixedOffset) -> Result<Self, Self::Error> {
        parse_east_fixed_offset(TimezoneSeconds(value.local_minus_utc())).map(|_| Self(value))
    }
}
#[derive(Debug, thiserror::Error, optml::Optml)]
pub enum TryFromStdEnvVarOkTimezoneError {
    #[error("{chrono_fixed_offset:?}")]
    ChronoFixedOffset {
        chrono_fixed_offset: ChronoFixedOffsetError,
    },
    #[error("{i32_parsing:?}")]
    I32Parsing { i32_parsing: StdI32ParsingError },
}
impl TryFromStdEnvVarOk for ChronoTimezone {
    type Error = TryFromStdEnvVarOkTimezoneError;
    fn try_from_std_env_var_ok(v: StdEnvVarOk) -> Result<Self, Self::Error> {
        let i32_v = TimezoneSeconds::from(parse_from_str_with_error::<i32, _, _>(
            StdEnvVarOkRef::from(v.0.as_str()),
            |i32_parsing| Self::Error::I32Parsing {
                i32_parsing: StdI32ParsingError::from(i32_parsing),
            },
        )?);
        parse_east_fixed_offset(i32_v)
            .map_err(|chrono_fixed_offset| Self::Error::ChronoFixedOffset {
                chrono_fixed_offset,
            })
            .map(Self::from)
    }
}
impl TryFromStdEnvVarOk for types::TracingFormat {
    type Error = std::convert::Infallible;
    fn try_from_std_env_var_ok(v: StdEnvVarOk) -> Result<Self, Self::Error> {
        Ok(if v.0.eq_ignore_ascii_case(str_constants::JSON) {
            Self::Json
        } else {
            Self::Text
        })
    }
}
#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum TryFromStdEnvVarOkSvcModeError {
    #[error("service mode must be migrate or serve")]
    Unknown,
}
impl TryFromStdEnvVarOk for types::SvcMode {
    type Error = TryFromStdEnvVarOkSvcModeError;
    fn try_from_std_env_var_ok(v: StdEnvVarOk) -> Result<Self, Self::Error> {
        match v.0.as_str() {
            str_constants::SERVICE_MODE_MIGRATE => Ok(Self::Migrate),
            str_constants::SERVICE_MODE_SERVE => Ok(Self::Serve),
            _unknown => Err(TryFromStdEnvVarOkSvcModeError::Unknown),
        }
    }
}
config_lib_macros::impl_try_from_parse_string_error!(
    TracingLevel,
    TryFromStdEnvVarOkTracingLevelError,
    types::TracingLevel,
    AppStateTracingLevelParsing,
    app_state_tracing_type_parsing
);
pub fn parse_required_env_var<T, ParseError, Error, MapEnvVarError, Parse, MapParseError>(
    env_var_name: EnvVarNameRef<'_>,
    map_env_var_error: MapEnvVarError,
    parse: Parse,
    map_parse_error: MapParseError,
) -> Result<T, Error>
where
    MapEnvVarError: FnOnce(std::env::VarError, EnvVarName) -> Error,
    Parse: FnOnce(StdEnvVarOk) -> Result<T, ParseError>,
    MapParseError: FnOnce(ParseError) -> Error,
{
    let v = std::env::var(env_var_name.0).map_err(|std_env_var_error| {
        map_env_var_error(
            std_env_var_error,
            EnvVarName::try_from(env_var_name.0.to_owned()).unwrap_or_else(EnvVarName::from),
        )
    })?;
    parse(StdEnvVarOk::try_from(v).unwrap_or_else(StdEnvVarOk::from)).map_err(map_parse_error)
}
fn try_map_non_empty_env_value<T, Error>(
    v: StdEnvVarOk,
    mk_error: impl FnOnce(&'static str) -> Error,
    map_ok: impl FnOnce(String) -> T,
) -> Result<T, Error> {
    if v.0.is_empty() {
        return Err(mk_error(str_constants::CONFIG_ENV_VALUE_IS_EMPTY_MSG));
    }
    Ok(map_ok(v.0))
}
fn parse_from_str_with_error<T, ParseError, Error>(
    v: StdEnvVarOkRef<'_>,
    mk_error: impl FnOnce(ParseError) -> Error,
) -> Result<T, Error>
where
    T: std::str::FromStr<Err = ParseError>,
{
    v.0.parse::<T>().map_err(mk_error)
}
fn parse_east_fixed_offset(
    v: TimezoneSeconds,
) -> Result<ChronoEastFixedOffset, ChronoFixedOffsetError> {
    chrono::FixedOffset::east_opt(v.0)
        .map(ChronoEastFixedOffset)
        .ok_or_else(|| ChronoFixedOffsetError::from(str_constants::CONFIG_TIMEZONE_NOT_EAST_MSG))
}
#[cfg(test)]
mod tests {
    #[test]
    fn svc_mode_accepts_only_documented_values() {
        assert_eq!(
            <super::types::SvcMode as super::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                super::StdEnvVarOk::try_from(str_constants::SERVICE_MODE_MIGRATE.to_owned())
                    .expect("39a8e94f"),
            ),
            Ok(super::types::SvcMode::Migrate)
        );
        assert_eq!(
            <super::types::SvcMode as super::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                super::StdEnvVarOk::try_from(str_constants::SERVICE_MODE_SERVE.to_owned())
                    .expect("045ca5a1"),
            ),
            Ok(super::types::SvcMode::Serve)
        );
        assert_eq!(
            <super::types::SvcMode as super::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                super::StdEnvVarOk::try_from(str_constants::INVALID_REQUEST.to_owned())
                    .expect("156cc47b"),
            ),
            Err(super::TryFromStdEnvVarOkSvcModeError::Unknown)
        );
    }
    #[derive(optml::Optml, Debug, PartialEq, Eq)]
    enum ParseRequiredEnvVarTestError {
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
    fn administrator_token_text_deserialization_uses_bounded_try_from() {
        let issuer_deserializer =
            serde::de::value::StringDeserializer::<serde::de::value::Error>::new(
                str_constants::TEST_JWT_SECRET_CHARACTER_A.repeat(257usize),
            );
        let audience_deserializer =
            serde::de::value::StringDeserializer::<serde::de::value::Error>::new(
                str_constants::TEST_JWT_SECRET_CHARACTER_A.repeat(257usize),
            );
        let Err(_issuer_error) =
            <super::AdminTokenIssuer as serde::Deserialize>::deserialize(issuer_deserializer)
        else {
            panic!("b286db7c");
        };
        let Err(_audience_error) =
            <super::AdminTokenAudience as serde::Deserialize>::deserialize(audience_deserializer)
        else {
            panic!("70f1e49f");
        };
    }
    #[test]
    fn cors_allow_origin_parsing_returns_value() {
        config_lib_macros::assert_parse_ok_matches!(
            super::CorsAllowOrigin,
            str_constants::ASTERISK,
            super::CorsAllowOrigin(_)
        );
    }
    #[test]
    fn cors_allow_origin_parsing_returns_error_for_empty_string() {
        config_lib_macros::assert_empty_parse_err_matches!(
            super::CorsAllowOrigin,
            super::TryFromStdEnvVarOkCorsAllowOriginError::IsEmpty { .. }
        );
    }
    #[test]
    fn database_url_parsing_returns_value_for_non_empty_input() {
        config_lib_macros::assert_parse_ok_matches!(
            super::DatabaseUrl,
            str_constants::POSTGRES_DB,
            super::DatabaseUrl(_)
        );
    }
    #[test]
    fn database_url_parsing_returns_error_for_empty_string() {
        config_lib_macros::assert_empty_parse_err_matches!(
            super::DatabaseUrl,
            super::TryFromStdEnvVarOkDatabaseUrlError::IsEmpty { .. }
        );
    }
    #[test]
    fn secret_url_debug_output_redacts_credentials() {
        let all_redacted = [
            str_constants::POSTGRES_USERNAME_LOCALHOST_TEST,
            str_constants::POSTGRES_USERNAME_PASSWORD_LOCALHOST_TEST_QUESTION_SSLMODE_DISABLE,
            str_constants::POSTGRES_PERCENT_PERCENT_40NAME_PERCENT_PERCENT_2FPASSWORD_PATH_1_TEST_FRAGMENT,
        ]
        .into_iter()
        .all(|raw| {
            let value = parse_env::<super::DatabaseUrl>(raw).expect("ae91f62c");
            let debug = format!("{value:?}");
            !debug.contains(raw)
                && !debug.contains(str_constants::USERNAME)
                && !debug.contains(str_constants::PASSWORD)
                && !debug.contains(str_constants::PERCENT_PERCENT_40NAME)
                && !debug.contains(str_constants::PERCENT_PERCENT_2FPASSWORD)
                && debug.contains(str_constants::REDACTED_ALT)
        });
        assert!(all_redacted);
    }
    #[test]
    fn mongo_url_parsing_returns_value_for_non_empty_input() {
        config_lib_macros::assert_parse_ok_matches!(
            super::MongoUrl,
            str_constants::MONGODB_DB,
            super::MongoUrl(_)
        );
    }
    #[test]
    fn mongo_url_parsing_returns_error_for_empty_string() {
        config_lib_macros::assert_empty_parse_err_matches!(
            super::MongoUrl,
            super::TryFromStdEnvVarOkMongoUrlError::IsEmpty { .. }
        );
    }
    #[test]
    fn redis_url_parsing_returns_value_for_non_empty_input() {
        config_lib_macros::assert_parse_ok_matches!(
            super::RedisUrl,
            str_constants::REDIS_DB,
            super::RedisUrl(_)
        );
    }
    #[test]
    fn redis_url_parsing_returns_error_for_empty_string() {
        config_lib_macros::assert_empty_parse_err_matches!(
            super::RedisUrl,
            super::TryFromStdEnvVarOkRedisUrlError::IsEmpty { .. }
        );
    }
    #[test]
    fn src_place_type_parsing_is_case_insensitive() {
        config_lib_macros::assert_parse_ok_matches!(
            super::SrcPlaceType,
            str_constants::GITHUB_ALT,
            super::SrcPlaceType(super::types::SrcPlaceType::Github)
        );
    }
    #[test]
    fn src_place_type_parsing_returns_error_for_unknown_value() {
        config_lib_macros::assert_parse_err_matches!(
            super::SrcPlaceType,
            str_constants::BAD,
            super::TryFromStdEnvVarOkSrcPlaceTypeError::AppStateSrcPlaceTypeParsing { .. }
        );
    }
    #[test]
    fn tracing_level_parsing_is_case_insensitive() {
        config_lib_macros::assert_parse_ok_matches!(
            super::TracingLevel,
            str_constants::DEBUG,
            super::TracingLevel(super::types::TracingLevel::Debug)
        );
    }
    #[test]
    fn tracing_level_parsing_returns_error_for_unknown_value() {
        config_lib_macros::assert_parse_err_matches!(
            super::TracingLevel,
            str_constants::BAD,
            super::TryFromStdEnvVarOkTracingLevelError::AppStateTracingLevelParsing { .. }
        );
    }
    #[test]
    fn enable_api_git_commit_check_parsing_returns_bool() {
        config_lib_macros::assert_parse_ok_matches!(
            super::EnableApiGitCommitCheck,
            str_constants::TRUE,
            super::EnableApiGitCommitCheck(true)
        );
    }
    #[test]
    fn enable_api_git_commit_check_parsing_returns_error_for_invalid_bool() {
        config_lib_macros::assert_parse_err_matches!(
            super::EnableApiGitCommitCheck,
            str_constants::TRUTHY,
            super::TryFromStdEnvVarOkEnableApiGitCommitCheckError::BoolParsing { .. }
        );
    }
    #[test]
    fn maximum_size_of_http_body_in_bytes_parsing_returns_usize() {
        let parsed = parse_env::<super::MaximumSizeOfHttpBodyInBytes>(str_constants::VALUE_128)
            .expect("d5b7a09e");
        assert_eq!(*parsed, 128usize);
    }
    #[test]
    fn maximum_size_of_http_body_in_bytes_parsing_returns_error_for_invalid_number() {
        config_lib_macros::assert_parse_err_matches!(
            super::MaximumSizeOfHttpBodyInBytes,
            str_constants::VALUE_1K,
            super::TryFromStdEnvVarOkMaximumSizeOfHttpBodyInBytesError::UsizeParsing { .. }
        );
    }
    #[test]
    fn maximum_size_of_http_body_in_bytes_parsing_returns_error_for_zero() {
        config_lib_macros::assert_parse_err_matches!(
            super::MaximumSizeOfHttpBodyInBytes,
            str_constants::VALUE_0,
            super::TryFromStdEnvVarOkMaximumSizeOfHttpBodyInBytesError::MaximumSizeOfHttpBodyInBytes { .. }
        );
    }
    #[test]
    fn pg_pool_max_connections_parsing_returns_u32() {
        let parsed =
            parse_env::<super::PgPoolMaxConnections>(str_constants::VALUE_10).expect("5d9032ac");
        assert_eq!(*parsed, 10u32);
    }
    #[test]
    fn pg_pool_max_connections_parsing_returns_error_for_invalid_number() {
        config_lib_macros::assert_parse_err_matches!(
            super::PgPoolMaxConnections,
            str_constants::BAD,
            super::TryFromStdEnvVarOkPgPoolMaxConnectionsError::U32Parsing { .. }
        );
    }
    #[test]
    fn pg_pool_max_connections_parsing_returns_error_for_zero() {
        config_lib_macros::assert_parse_err_matches!(
            super::PgPoolMaxConnections,
            str_constants::VALUE_0,
            super::TryFromStdEnvVarOkPgPoolMaxConnectionsError::PgPoolMaxConnections { .. }
        );
    }
    #[test]
    fn non_empty_string_parser_returns_error_for_empty_value() {
        config_lib_macros::assert_empty_parse_err_matches!(
            super::StartingCheckLink,
            super::TryFromStdEnvVarOkStartingCheckLinkError::IsEmpty { .. }
        );
    }
    #[test]
    fn non_empty_string_parser_returns_value_for_non_empty_value() {
        config_lib_macros::assert_parse_ok_matches!(
            super::StartingCheckLink,
            str_constants::HTTPS_EXAMPLE_COM,
            super::StartingCheckLink(_)
        );
    }
    #[test]
    fn service_socket_address_parsing_returns_socket_addr() {
        config_lib_macros::assert_parse_ok_matches!(
            super::ServiceSocketAddress,
            str_constants::VALUE_127_0_0_1_3000,
            super::ServiceSocketAddress(_)
        );
    }
    #[test]
    fn service_socket_address_parsing_returns_error_for_invalid_addr() {
        let error = parse_env::<super::ServiceSocketAddress>(str_constants::VALUE_127_0_0_1);
        assert!(matches!(
            error,
            Err(super::TryFromStdEnvVarOkServiceSocketAddressError::StdNetSocketAddr { .. })
        ));
    }
    #[test]
    fn timezone_parsing_returns_timezone_for_valid_offset() {
        config_lib_macros::assert_parse_ok_matches!(
            super::ChronoTimezone,
            str_constants::VALUE_0,
            super::ChronoTimezone(_)
        );
    }
    #[test]
    fn timezone_parsing_returns_i32_error_for_non_number() {
        config_lib_macros::assert_parse_err_matches!(
            super::ChronoTimezone,
            str_constants::NAN,
            super::TryFromStdEnvVarOkTimezoneError::I32Parsing { .. }
        );
    }
    #[test]
    fn parse_east_fixed_offset_returns_offset_for_valid_seconds() {
        let parsed = super::parse_east_fixed_offset(super::TimezoneSeconds::from(3i32 * 3_600i32));
        assert!(matches!(parsed, Ok(v) if v.0.local_minus_utc() == 3i32 * 3_600i32));
    }
    #[test]
    fn parse_east_fixed_offset_returns_error_for_out_of_range_seconds() {
        let parsed = super::parse_east_fixed_offset(super::TimezoneSeconds::from(i32::MAX));
        assert_eq!(
            parsed,
            Err(super::ChronoFixedOffsetError(
                str_constants::CONFIG_TIMEZONE_NOT_EAST_MSG,
            ))
        );
    }
    #[test]
    fn timezone_parsing_returns_offset_error_when_out_of_range() {
        let out_of_range = i32::MAX.to_string();
        let error = parse_env::<super::ChronoTimezone>(&out_of_range);
        assert!(matches!(
            error,
            Err(super::TryFromStdEnvVarOkTimezoneError::ChronoFixedOffset { .. })
        ));
    }
    #[test]
    fn parse_required_env_var_parses_value_when_env_var_exists() {
        let parsed = super::parse_required_env_var(
            super::EnvVarNameRef::from(str_constants::PATH_ALT),
            |_std_env_var_error, env_var_name| ParseRequiredEnvVarTestError::EnvVar {
                env_var_name,
            },
            |v| Ok::<_, &'static str>(v.0.len()),
            |parse| ParseRequiredEnvVarTestError::Parse { parse },
        );
        assert!(matches!(parsed, Ok(v) if v > 0));
    }
    #[test]
    fn parse_required_env_var_maps_missing_env_var_error() {
        let parsed = super::parse_required_env_var(
            super::EnvVarNameRef::from(str_constants::CONFIG_LIB_TEST_ENV_VAR_4E8A7F21),
            |_std_env_var_error, env_var_name| ParseRequiredEnvVarTestError::EnvVar {
                env_var_name,
            },
            Ok::<_, &'static str>,
            |parse| ParseRequiredEnvVarTestError::Parse { parse },
        );
        assert_eq!(
            parsed,
            Err(ParseRequiredEnvVarTestError::EnvVar {
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
            super::EnvVarNameRef::from(str_constants::PATH_ALT),
            |_std_env_var_error, env_var_name| ParseRequiredEnvVarTestError::EnvVar {
                env_var_name,
            },
            |_v| Err::<(), _>(str_constants::PARSE_FAILED),
            |parse| ParseRequiredEnvVarTestError::Parse { parse },
        );
        assert_eq!(
            parsed,
            Err(ParseRequiredEnvVarTestError::Parse {
                parse: "parse failed"
            })
        );
    }
}
