#![allow(clippy::arbitrary_source_item_ordering)] // configuration declarations stay grouped with their parse errors and TryFromStdEnvVarOk implementations
mod admin;
mod admin_jwt;
mod bool_flags;
mod http;
mod pg_pool;
pub mod types;
pub use admin::{
    AdminAccessTokenTtlSeconds, AdminAccessTokenTtlSecondsProvider, AdminLoginFailureLimit,
    AdminLoginFailureLimitProvider, AdminPasswordHashConcurrency,
    AdminPasswordHashConcurrencyProvider, AdminPositiveU64ParsingError,
    AdminPositiveUsizeParsingError, AdminRefreshTokenTtlSeconds,
    AdminRefreshTokenTtlSecondsProvider, AdminSessionLimit, AdminSessionLimitProvider,
    AdminSignInRateLimit, AdminSignInRateLimitProvider, AdminTokenAudience,
    AdminTokenAudienceProvider, AdminTokenAudienceTryFromStringError, AdminTokenIssuer,
    AdminTokenIssuerProvider, AdminTokenIssuerTryFromStringError,
    TryFromStdEnvVarOkAdminPasswordHashConcurrencyError, TryFromStdEnvVarOkAdminPositiveU64Error,
    TryFromStdEnvVarOkAdminTokenTextError,
};
pub use admin_jwt::{
    AdminJwtSecret, AdminJwtSecretProvider, TryFromStdEnvVarOkAdminJwtSecretError,
};
pub use bool_flags::{
    AdminBoolParsingError, AdminCookieSecure, AdminCookieSecureProvider, AdminSwaggerEnabled,
    AdminSwaggerEnabledProvider, HttpGzipEnabled, ProductionMode,
    TryFromStdEnvVarOkAdminCookieSecureError,
};
pub use http::{
    ContentSecurityPolicy, ContentSecurityPolicyError, MaximumSizeOfHttpBodyInBytes,
    MaximumSizeOfHttpBodyInBytesProvider, MaximumSizeOfHttpBodyInBytesTryFromUsizeError,
    TryFromStdEnvVarOkMaximumSizeOfHttpBodyInBytesError,
};
pub use pg_pool::{
    PgPoolAcquireTimeoutSeconds, PgPoolConfigParseError, PgPoolIdleTimeoutSeconds,
    PgPoolMaxConnections, PgPoolMaxConnectionsProvider, PgPoolMaxConnectionsTryFromU32Error,
    PgPoolMaxLifetimeSeconds, PgPoolMinConnections, RequestTimeoutSeconds,
    TryFromStdEnvVarOkPgPoolMaxConnectionsError,
};
const CONFIG_LIB_STRING_WRAPPER_MAX_LEN: usize = 1_048_576;
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, PartialEq, Eq)]
pub struct StdEnvVarOk(String);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, PartialEq, Eq, thiserror::Error,
)]
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
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::FromInner)]
pub struct StdEnvVarOkRef<'value_lt>(&'value_lt str);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::AsRefInner,
    newtype::FromInner,
)]
pub struct EnvVarNameRef<'name_lt>(&'name_lt str);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, PartialEq, Eq, newtype::Display,
)]
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
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    newtype::FromInner,
)]
pub struct ChronoFixedOffsetError(&'static str);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, newtype::DebugTransparent, newtype::FromInner,
)]
pub struct I32ParseIntError(std::num::ParseIntError);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, newtype::DebugTransparent, newtype::FromInner,
)]
pub struct U32ParseIntError(std::num::ParseIntError);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, newtype::DebugTransparent, newtype::FromInner,
)]
pub struct UsizeParseIntError(std::num::ParseIntError);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    newtype::FromInner,
)]
struct TimezoneSeconds(i32);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    newtype::FromInner,
)]
struct ChronoEastFixedOffset(chrono::FixedOffset);
pub trait TryFromStdEnvVarOk: Sized {
    type Error;
    fn try_from_std_env_var_ok(v: StdEnvVarOk) -> Result<Self, Self::Error>;
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConfigFieldSensitivity {
    Public,
    Secret,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConfigFieldRequirement {
    Required,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConfigExampleValidity {
    Invalid,
    Valid,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::AsRefInner,
    newtype::FromInner,
)]
pub struct ConfigFieldExampleRef<'example_lt>(&'example_lt str);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::AsRefInner,
    newtype::FromInner,
)]
pub struct ConfigRustTypeName(&'static str);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
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
        f.debug_struct(constants_str::CONFIG_FIELD_DESCRIPTOR)
            .field(constants_str::ENV_NAME, &self.env_name)
            .field(constants_str::EXAMPLE, &self.example)
            .field(constants_str::REQUIRED, &self.requirement)
            .field(constants_str::RUST_TYPE_NAME, &self.rust_type_name)
            .field(constants_str::SENSITIVITY, &self.sensitivity)
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
    optimal_memory_layout::OptimalMemoryLayout,
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
#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::AsRefOwned, newtype::FromInner)]
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
        f.write_str(constants_str::REDACTED_ALT_3)
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    newtype::DerefInner,
    newtype::FromInner,
)]
pub struct ConfigNonZeroU64(std::num::NonZeroU64);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    newtype::DerefInner,
    newtype::FromInner,
)]
pub struct ConfigNonZeroUsize(std::num::NonZeroUsize);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, newtype::DebugTransparent, newtype::FromInner,
)]
pub struct ParseIntError(std::num::ParseIntError);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, newtype::DebugTransparent, newtype::FromInner,
)]
pub struct ParseBoolError(std::str::ParseBoolError);
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
    generate_accessor_traits_for_struct_fields::GenerateAccessorTrait,
    optimal_memory_layout::OptimalMemoryLayout,
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
#[derive(Debug, thiserror::Error, optimal_memory_layout::OptimalMemoryLayout)]
pub enum TryFromStdEnvVarOkTimezoneError {
    #[error("{chrono_fixed_offset:?}")]
    ChronoFixedOffset {
        chrono_fixed_offset: ChronoFixedOffsetError,
    },
    #[error("{i32_parsing:?}")]
    I32Parsing { i32_parsing: I32ParseIntError },
}
impl TryFromStdEnvVarOk for ChronoTimezone {
    type Error = TryFromStdEnvVarOkTimezoneError;
    fn try_from_std_env_var_ok(v: StdEnvVarOk) -> Result<Self, Self::Error> {
        let i32_v = TimezoneSeconds::from(parse_from_str_with_error::<i32, _, _>(
            StdEnvVarOkRef::from(v.0.as_str()),
            |i32_parsing| Self::Error::I32Parsing {
                i32_parsing: I32ParseIntError::from(i32_parsing),
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
        Ok(if v.0.eq_ignore_ascii_case(constants_str::JSON) {
            Self::Json
        } else {
            Self::Text
        })
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum TryFromStdEnvVarOkSvcModeError {
    #[error("service mode must be migrate or serve")]
    Unknown,
}
impl TryFromStdEnvVarOk for types::SvcMode {
    type Error = TryFromStdEnvVarOkSvcModeError;
    fn try_from_std_env_var_ok(v: StdEnvVarOk) -> Result<Self, Self::Error> {
        match v.0.as_str() {
            constants_str::SERVICE_MODE_MIGRATE => Ok(Self::Migrate),
            constants_str::SERVICE_MODE_SERVE => Ok(Self::Serve),
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
        return Err(mk_error(constants_str::CONFIG_ENV_VALUE_IS_EMPTY_MSG));
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
        .ok_or_else(|| ChronoFixedOffsetError::from(constants_str::CONFIG_TIMEZONE_NOT_EAST_MSG))
}
#[cfg(test)]
mod tests;
