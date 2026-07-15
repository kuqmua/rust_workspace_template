#![allow(clippy::arbitrary_source_item_ordering)] // configuration declarations stay grouped with their parse errors and TryFromStdEnvVarOk implementations
pub mod types;
const CONFIG_LIB_STRING_WRAPPER_MAX_LEN: usize = 1_048_576;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StdEnvVarOk(String);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigLibStringWrapperTryFromStringError {
    TooLong { len: usize, max: usize },
}
impl std::fmt::Display for ConfigLibStringWrapperTryFromStringError {
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
#[derive(Debug, Clone, Copy, newtype::Newtype)]
#[newtype(from_inner)]
pub struct StdEnvVarOkRef<'value_lt>(&'value_lt str);
#[derive(Debug, Clone, Copy, newtype::Newtype)]
#[newtype(from_inner)]
pub struct EnvVarNameRef<'name_lt>(&'name_lt str);
#[derive(Debug, Clone, PartialEq, Eq, newtype::Newtype)]
#[newtype(display)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChronoFixedOffsetError(&'static str);
#[derive(newtype::Newtype)]
#[newtype(debug_transparent)]
pub struct StdI32ParsingError(std::num::ParseIntError);
#[derive(newtype::Newtype)]
#[newtype(debug_transparent)]
pub struct StdU32ParsingError(std::num::ParseIntError);
#[derive(newtype::Newtype)]
#[newtype(debug_transparent)]
pub struct StdUsizeParsingError(std::num::ParseIntError);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TimezoneSeconds(i32);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ChronoEastFixedOffset(chrono::FixedOffset);
pub trait TryFromStdEnvVarOk: Sized {
    type Error;
    fn try_from_std_env_var_ok(v: StdEnvVarOk) -> Result<Self, Self::Error>;
}
const ADMIN_JWT_SECRET_MIN_LEN: usize = 32;
#[derive(newtype::Newtype)]
#[newtype(as_ref_owned, from_inner)]
pub struct SecrecySecretBoxString(secrecy::SecretBox<String>);
impl std::fmt::Debug for SecrecySecretBoxString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[REDACTED]")
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, newtype::Newtype)]
#[newtype(deref_inner, from_inner)]
pub struct StdNonZeroU64(std::num::NonZeroU64);
#[derive(Debug, Clone, Copy, PartialEq, Eq, newtype::Newtype)]
#[newtype(deref_inner, from_inner)]
pub struct StdNonZeroUsize(std::num::NonZeroUsize);
#[derive(newtype::Newtype)]
#[newtype(debug_transparent, from_inner)]
pub struct StdParseIntError(std::num::ParseIntError);
#[derive(newtype::Newtype)]
#[newtype(debug_transparent, from_inner)]
pub struct StdParseBoolError(std::str::ParseBoolError);
#[derive(generate_getter_traits_for_struct_fields::GenerateGetterTrait, newtype::Newtype)]
#[newtype(as_ref_owned)]
pub struct AdminJwtSecret(SecrecySecretBoxString);
impl std::fmt::Debug for AdminJwtSecret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("AdminJwtSecret")
            .field(&"[REDACTED]")
            .finish()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum TryFromStdEnvVarOkAdminJwtSecretError {
    #[error("administrator JWT secret must contain at least {ADMIN_JWT_SECRET_MIN_LEN} bytes")]
    TooShort,
}
impl TryFromStdEnvVarOk for AdminJwtSecret {
    type Error = TryFromStdEnvVarOkAdminJwtSecretError;
    fn try_from_std_env_var_ok(v: StdEnvVarOk) -> Result<Self, Self::Error> {
        if v.0.len() < ADMIN_JWT_SECRET_MIN_LEN {
            Err(Self::Error::TooShort)
        } else {
            Ok(Self(SecrecySecretBoxString::from(secrecy::SecretBox::new(
                Box::new(v.0),
            ))))
        }
    }
}
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    generate_getter_traits_for_struct_fields::GenerateGetterTrait,
    newtype::Newtype,
)]
#[newtype(deref_inner)]
pub struct AdminAccessTokenTtlSeconds(StdNonZeroU64);
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    generate_getter_traits_for_struct_fields::GenerateGetterTrait,
    newtype::Newtype,
)]
#[newtype(deref_inner)]
pub struct AdminRefreshTokenTtlSeconds(StdNonZeroU64);
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    generate_getter_traits_for_struct_fields::GenerateGetterTrait,
    newtype::Newtype,
)]
#[newtype(deref_inner)]
pub struct AdminSignInRateLimit(StdNonZeroU64);
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    generate_getter_traits_for_struct_fields::GenerateGetterTrait,
    newtype::Newtype,
)]
#[newtype(deref_inner)]
pub struct AdminSessionLimit(StdNonZeroUsize);
#[derive(newtype::Newtype)]
#[newtype(debug_transparent)]
pub struct AdminPositiveU64ParsingError(StdParseIntError);
#[derive(Debug, thiserror::Error)]
pub enum TryFromStdEnvVarOkAdminPositiveU64Error {
    #[error("administrator duration must be greater than zero")]
    IsZero,
    #[error("{admin_positive_u64_parsing:?}")]
    Parse {
        admin_positive_u64_parsing: AdminPositiveU64ParsingError,
    },
}
fn parse_admin_positive_u64(
    v: &StdEnvVarOk,
) -> Result<StdNonZeroU64, TryFromStdEnvVarOkAdminPositiveU64Error> {
    let parsed = v.0.parse::<u64>().map_err(|admin_positive_u64_parsing| {
        TryFromStdEnvVarOkAdminPositiveU64Error::Parse {
            admin_positive_u64_parsing: AdminPositiveU64ParsingError(StdParseIntError::from(
                admin_positive_u64_parsing,
            )),
        }
    })?;
    std::num::NonZeroU64::new(parsed)
        .map(StdNonZeroU64::from)
        .ok_or(TryFromStdEnvVarOkAdminPositiveU64Error::IsZero)
}
impl TryFromStdEnvVarOk for AdminAccessTokenTtlSeconds {
    type Error = TryFromStdEnvVarOkAdminPositiveU64Error;
    fn try_from_std_env_var_ok(v: StdEnvVarOk) -> Result<Self, Self::Error> {
        parse_admin_positive_u64(&v).map(Self)
    }
}
impl TryFromStdEnvVarOk for AdminRefreshTokenTtlSeconds {
    type Error = TryFromStdEnvVarOkAdminPositiveU64Error;
    fn try_from_std_env_var_ok(v: StdEnvVarOk) -> Result<Self, Self::Error> {
        parse_admin_positive_u64(&v).map(Self)
    }
}
impl TryFromStdEnvVarOk for AdminSignInRateLimit {
    type Error = TryFromStdEnvVarOkAdminPositiveU64Error;
    fn try_from_std_env_var_ok(v: StdEnvVarOk) -> Result<Self, Self::Error> {
        parse_admin_positive_u64(&v).map(Self)
    }
}
impl TryFromStdEnvVarOk for AdminSessionLimit {
    type Error = TryFromStdEnvVarOkAdminPositiveU64Error;
    fn try_from_std_env_var_ok(v: StdEnvVarOk) -> Result<Self, Self::Error> {
        let value = parse_admin_positive_u64(&v)?;
        usize::try_from(value.0.get())
            .ok()
            .and_then(std::num::NonZeroUsize::new)
            .map(StdNonZeroUsize::from)
            .map(Self)
            .ok_or(TryFromStdEnvVarOkAdminPositiveU64Error::IsZero)
    }
}
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    generate_getter_traits_for_struct_fields::GenerateGetterTrait,
    newtype::Newtype,
)]
#[newtype(deref_inner)]
pub struct AdminPasswordHashConcurrency(StdNonZeroUsize);
#[derive(newtype::Newtype)]
#[newtype(debug_transparent)]
pub struct AdminPositiveUsizeParsingError(StdParseIntError);
#[derive(Debug, thiserror::Error)]
pub enum TryFromStdEnvVarOkAdminPasswordHashConcurrencyError {
    #[error("administrator password hash concurrency must be greater than zero")]
    IsZero,
    #[error("{admin_positive_usize_parsing:?}")]
    Parse {
        admin_positive_usize_parsing: AdminPositiveUsizeParsingError,
    },
}
impl TryFromStdEnvVarOk for AdminPasswordHashConcurrency {
    type Error = TryFromStdEnvVarOkAdminPasswordHashConcurrencyError;
    fn try_from_std_env_var_ok(v: StdEnvVarOk) -> Result<Self, Self::Error> {
        let parsed =
            v.0.parse::<usize>()
                .map_err(|admin_positive_usize_parsing| Self::Error::Parse {
                    admin_positive_usize_parsing: AdminPositiveUsizeParsingError(
                        StdParseIntError::from(admin_positive_usize_parsing),
                    ),
                })?;
        std::num::NonZeroUsize::new(parsed)
            .map(StdNonZeroUsize::from)
            .map(Self)
            .ok_or(Self::Error::IsZero)
    }
}
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    generate_getter_traits_for_struct_fields::GenerateGetterTrait,
    newtype::Newtype,
)]
#[newtype(deref_inner)]
pub struct AdminCookieSecure(bool);
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    generate_getter_traits_for_struct_fields::GenerateGetterTrait,
    newtype::Newtype,
)]
#[newtype(deref_inner)]
pub struct AdminSwaggerEnabled(bool);
#[derive(newtype::Newtype)]
#[newtype(debug_transparent)]
pub struct AdminBoolParsingError(StdParseBoolError);
#[derive(Debug, thiserror::Error)]
#[error("{admin_bool_parsing:?}")]
pub struct TryFromStdEnvVarOkAdminCookieSecureError {
    admin_bool_parsing: AdminBoolParsingError,
}
impl TryFromStdEnvVarOk for AdminCookieSecure {
    type Error = TryFromStdEnvVarOkAdminCookieSecureError;
    fn try_from_std_env_var_ok(v: StdEnvVarOk) -> Result<Self, Self::Error> {
        v.0.parse::<bool>()
            .map(Self)
            .map_err(|admin_bool_parsing| Self::Error {
                admin_bool_parsing: AdminBoolParsingError(StdParseBoolError::from(
                    admin_bool_parsing,
                )),
            })
    }
}
impl TryFromStdEnvVarOk for AdminSwaggerEnabled {
    type Error = TryFromStdEnvVarOkAdminCookieSecureError;
    fn try_from_std_env_var_ok(v: StdEnvVarOk) -> Result<Self, Self::Error> {
        v.0.parse::<bool>()
            .map(Self)
            .map_err(|admin_bool_parsing| Self::Error {
                admin_bool_parsing: AdminBoolParsingError(StdParseBoolError::from(
                    admin_bool_parsing,
                )),
            })
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    generate_getter_traits_for_struct_fields::GenerateGetterTrait,
    newtype::BoundedString,
    newtype::Newtype,
)]
#[bounded_string(max = 256, description = "administrator token issuer")]
#[newtype(as_ref_owned)]
pub struct AdminTokenIssuer(String);
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    generate_getter_traits_for_struct_fields::GenerateGetterTrait,
    newtype::BoundedString,
    newtype::Newtype,
)]
#[bounded_string(max = 256, description = "administrator token audience")]
#[newtype(as_ref_owned)]
pub struct AdminTokenAudience(String);
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum TryFromStdEnvVarOkAdminTokenTextError {
    #[error("administrator token text is empty")]
    Empty,
    #[error("administrator token text is too long")]
    TooLong,
}
fn parse_admin_token_text<T, Error>(
    v: StdEnvVarOk,
    map: impl FnOnce(String) -> Result<T, Error>,
) -> Result<T, TryFromStdEnvVarOkAdminTokenTextError> {
    if v.0.is_empty() {
        return Err(TryFromStdEnvVarOkAdminTokenTextError::Empty);
    }
    map(v.0).map_err(|_bounded_string_error| TryFromStdEnvVarOkAdminTokenTextError::TooLong)
}
impl TryFromStdEnvVarOk for AdminTokenIssuer {
    type Error = TryFromStdEnvVarOkAdminTokenTextError;
    fn try_from_std_env_var_ok(v: StdEnvVarOk) -> Result<Self, Self::Error> {
        parse_admin_token_text(v, Self::try_from)
    }
}
impl TryFromStdEnvVarOk for AdminTokenAudience {
    type Error = TryFromStdEnvVarOkAdminTokenTextError;
    fn try_from_std_env_var_ok(v: StdEnvVarOk) -> Result<Self, Self::Error> {
        parse_admin_token_text(v, Self::try_from)
    }
}
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
#[derive(
    Debug,
    Clone,
    Copy,
    generate_getter_traits_for_struct_fields::GenerateGetterTrait,
    optml::Optml,
    newtype::Newtype,
)]
#[newtype(deref_inner)]
pub struct MaximumSizeOfHttpBodyInBytes(usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error, optml::Optml)]
pub enum MaximumSizeOfHttpBodyInBytesTryFromUsizeError {
    #[error("maximum size of http body in bytes must be greater than zero")]
    IsZero,
}
impl TryFrom<usize> for MaximumSizeOfHttpBodyInBytes {
    type Error = MaximumSizeOfHttpBodyInBytesTryFromUsizeError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(Self::Error::IsZero)
        } else {
            Ok(Self(value))
        }
    }
}
#[derive(Debug, thiserror::Error, optml::Optml)]
pub enum TryFromStdEnvVarOkMaximumSizeOfHttpBodyInBytesError {
    #[error("{maximum_size_of_http_body_in_bytes:?}")]
    MaximumSizeOfHttpBodyInBytes {
        maximum_size_of_http_body_in_bytes: MaximumSizeOfHttpBodyInBytesTryFromUsizeError,
    },
    #[error("{:?}", .usize_parsing)]
    UsizeParsing { usize_parsing: StdUsizeParsingError },
}
impl TryFromStdEnvVarOk for MaximumSizeOfHttpBodyInBytes {
    type Error = TryFromStdEnvVarOkMaximumSizeOfHttpBodyInBytesError;
    fn try_from_std_env_var_ok(v: StdEnvVarOk) -> Result<Self, Self::Error> {
        let parsed: usize =
            parse_from_str_with_error(StdEnvVarOkRef(v.0.as_str()), |usize_parsing| {
                Self::Error::UsizeParsing {
                    usize_parsing: StdUsizeParsingError(usize_parsing),
                }
            })?;
        Self::try_from(parsed).map_err(|maximum_size_of_http_body_in_bytes| {
            Self::Error::MaximumSizeOfHttpBodyInBytes {
                maximum_size_of_http_body_in_bytes,
            }
        })
    }
}
config_lib_macros::impl_try_from_secret_url!(MongoUrl, TryFromStdEnvVarOkMongoUrlError);
#[derive(
    Debug,
    Clone,
    Copy,
    generate_getter_traits_for_struct_fields::GenerateGetterTrait,
    optml::Optml,
    newtype::Newtype,
)]
#[newtype(deref_inner)]
pub struct PgPoolMaxConnections(u32);
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error, optml::Optml)]
pub enum PgPoolMaxConnectionsTryFromU32Error {
    #[error("pg pool max connections must be greater than zero")]
    IsZero,
}
impl TryFrom<u32> for PgPoolMaxConnections {
    type Error = PgPoolMaxConnectionsTryFromU32Error;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(Self::Error::IsZero)
        } else {
            Ok(Self(value))
        }
    }
}
#[derive(Debug, thiserror::Error, optml::Optml)]
pub enum TryFromStdEnvVarOkPgPoolMaxConnectionsError {
    #[error("{pg_pool_max_connections:?}")]
    PgPoolMaxConnections {
        pg_pool_max_connections: PgPoolMaxConnectionsTryFromU32Error,
    },
    #[error("{:?}", .u32_parsing)]
    U32Parsing { u32_parsing: StdU32ParsingError },
}
impl TryFromStdEnvVarOk for PgPoolMaxConnections {
    type Error = TryFromStdEnvVarOkPgPoolMaxConnectionsError;
    fn try_from_std_env_var_ok(v: StdEnvVarOk) -> Result<Self, Self::Error> {
        let parsed: u32 = parse_from_str_with_error(StdEnvVarOkRef(v.0.as_str()), |u32_parsing| {
            Self::Error::U32Parsing {
                u32_parsing: StdU32ParsingError(u32_parsing),
            }
        })?;
        Self::try_from(parsed).map_err(|pg_pool_max_connections| {
            Self::Error::PgPoolMaxConnections {
                pg_pool_max_connections,
            }
        })
    }
}
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
    newtype::Newtype,
)]
#[newtype(deref_inner)]
pub struct ChronoTimezone(chrono::FixedOffset);
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
        let i32_v = TimezoneSeconds(parse_from_str_with_error(
            StdEnvVarOkRef(v.0.as_str()),
            |i32_parsing| Self::Error::I32Parsing {
                i32_parsing: StdI32ParsingError(i32_parsing),
            },
        )?);
        parse_east_fixed_offset(i32_v)
            .map_err(|chrono_fixed_offset| Self::Error::ChronoFixedOffset {
                chrono_fixed_offset,
            })
            .map(|timezone| Self(timezone.0))
    }
}
config_lib_macros::impl_try_from_parse_string_error!(
    TracingLevel,
    TryFromStdEnvVarOkTracingLevelError,
    types::TracingLevel,
    AppStateTracingLevelParsing,
    app_state_tracing_type_parsing
);
#[allow(clippy::single_call_fn)] // shared helper centralizes env var read + parse + error mapping for TryFromEnv derive output
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
        return Err(mk_error(str_constants::config::ENV_VALUE_IS_EMPTY_MSG));
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
#[allow(clippy::single_call_fn)] // extracted timezone conversion keeps conversion + message mapping reusable and directly testable
fn parse_east_fixed_offset(
    v: TimezoneSeconds,
) -> Result<ChronoEastFixedOffset, ChronoFixedOffsetError> {
    chrono::FixedOffset::east_opt(v.0)
        .map(ChronoEastFixedOffset)
        .ok_or(ChronoFixedOffsetError(
            str_constants::config::TIMEZONE_NOT_EAST_MSG,
        ))
}
#[cfg(test)]
mod tests {
    #[derive(Debug, PartialEq, Eq)]
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
            super::TryFromStdEnvVarOkCorsAllowOriginError::IsEmpty { .. }
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
            super::TryFromStdEnvVarOkDatabaseUrlError::IsEmpty { .. }
        );
    }
    #[test]
    fn secret_url_debug_output_redacts_credentials() {
        let all_redacted = [
            "postgres://username@localhost/test",
            "postgres://username:password@localhost/test?sslmode=disable",
            "postgres://percent%40name:percent%2Fpassword@[::1]/test#fragment",
        ]
        .into_iter()
        .all(|raw| {
            let value = parse_env::<super::DatabaseUrl>(raw).expect("ae91f62c");
            let debug = format!("{value:?}");
            !debug.contains(raw)
                && !debug.contains("username")
                && !debug.contains("password")
                && !debug.contains("percent%40name")
                && !debug.contains("percent%2Fpassword")
                && debug.contains("REDACTED")
        });
        assert!(all_redacted);
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
            super::TryFromStdEnvVarOkMongoUrlError::IsEmpty { .. }
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
            super::TryFromStdEnvVarOkRedisUrlError::IsEmpty { .. }
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
            super::TryFromStdEnvVarOkSrcPlaceTypeError::AppStateSrcPlaceTypeParsing { .. }
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
            super::TryFromStdEnvVarOkTracingLevelError::AppStateTracingLevelParsing { .. }
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
            super::TryFromStdEnvVarOkEnableApiGitCommitCheckError::BoolParsing { .. }
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
            super::TryFromStdEnvVarOkMaximumSizeOfHttpBodyInBytesError::UsizeParsing { .. }
        );
    }
    #[test]
    fn maximum_size_of_http_body_in_bytes_parsing_returns_error_for_zero() {
        config_lib_macros::assert_parse_err_matches!(
            super::MaximumSizeOfHttpBodyInBytes,
            "0",
            super::TryFromStdEnvVarOkMaximumSizeOfHttpBodyInBytesError::MaximumSizeOfHttpBodyInBytes { .. }
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
            super::TryFromStdEnvVarOkPgPoolMaxConnectionsError::U32Parsing { .. }
        );
    }
    #[test]
    fn pg_pool_max_connections_parsing_returns_error_for_zero() {
        config_lib_macros::assert_parse_err_matches!(
            super::PgPoolMaxConnections,
            "0",
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
        let error = parse_env::<super::ServiceSocketAddress>("127.0.0.1");
        assert!(matches!(
            error,
            Err(super::TryFromStdEnvVarOkServiceSocketAddressError::StdNetSocketAddr { .. })
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
            super::TryFromStdEnvVarOkTimezoneError::I32Parsing { .. }
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
            Err(super::ChronoFixedOffsetError(
                str_constants::config::TIMEZONE_NOT_EAST_MSG,
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
            super::EnvVarNameRef("PATH"),
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
            super::EnvVarNameRef("CONFIG_LIB_TEST_ENV_VAR_4E8A7F21"),
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
            super::EnvVarNameRef("PATH"),
            |_std_env_var_error, env_var_name| ParseRequiredEnvVarTestError::EnvVar {
                env_var_name,
            },
            |_v| Err::<(), _>("parse failed"),
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
