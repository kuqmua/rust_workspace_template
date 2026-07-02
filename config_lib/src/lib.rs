pub mod types;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoolConfigValue {
    False,
    True,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigParseStatus {
    Invalid,
    Valid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnvVarName {
    CorsAllowOrigin,
    DatabaseUrl,
    EnableApiGitCommitCheck,
    MaximumSizeOfHttpBodyInBytes,
    MongoUrl,
    PgPoolMaxConnections,
    RedisUrl,
    ServiceSocketAddress,
    SrcPlaceType,
    StartingCheckLink,
    Timezone,
    TracingLevel,
    Unknown,
}

impl EnvVarName {
    #[must_use]
    pub fn from_identifier_text<IdentifierText>(identifier_text: &IdentifierText) -> Self
    where
        IdentifierText: AsRef<str> + ?Sized,
    {
        match identifier_text.as_ref() {
            naming_constants::CONFIG_FIELD_CORS_ALLOW_ORIGIN => Self::CorsAllowOrigin,
            naming_constants::CONFIG_FIELD_DATABASE_URL => Self::DatabaseUrl,
            naming_constants::CONFIG_FIELD_ENABLE_API_GIT_COMMIT_CHECK => {
                Self::EnableApiGitCommitCheck
            }
            naming_constants::CONFIG_FIELD_MAXIMUM_SIZE_OF_HTTP_BODY_IN_BYTES => {
                Self::MaximumSizeOfHttpBodyInBytes
            }
            naming_constants::CONFIG_FIELD_MONGO_URL => Self::MongoUrl,
            naming_constants::CONFIG_FIELD_PG_POOL_MAX_CONNECTIONS => Self::PgPoolMaxConnections,
            naming_constants::CONFIG_FIELD_REDIS_URL => Self::RedisUrl,
            naming_constants::CONFIG_FIELD_SERVICE_SOCKET_ADDRESS => Self::ServiceSocketAddress,
            naming_constants::CONFIG_FIELD_SRC_PLACE_TYPE => Self::SrcPlaceType,
            naming_constants::CONFIG_FIELD_STARTING_CHECK_LINK => Self::StartingCheckLink,
            naming_constants::CONFIG_FIELD_TIMEZONE => Self::Timezone,
            naming_constants::CONFIG_FIELD_TRACING_LEVEL => Self::TracingLevel,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TryFromEnvError {
    MissingEnvVarName { env_var_name: EnvVarName },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TryFromStdEnvVarOkCorsAllowOriginError {
    MissingEnvVarName { env_var_name: EnvVarName },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TryFromStdEnvVarOkDatabaseUrlError {
    MissingEnvVarName { env_var_name: EnvVarName },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TryFromStdEnvVarOkEnableApiGitCommitCheckError {
    MissingEnvVarName { env_var_name: EnvVarName },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TryFromStdEnvVarOkMaximumSizeOfHttpBodyInBytesError {
    MissingEnvVarName { env_var_name: EnvVarName },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TryFromStdEnvVarOkMongoUrlError {
    MissingEnvVarName { env_var_name: EnvVarName },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TryFromStdEnvVarOkPgPoolMaxConnectionsError {
    MissingEnvVarName { env_var_name: EnvVarName },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TryFromStdEnvVarOkRedisUrlError {
    MissingEnvVarName { env_var_name: EnvVarName },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TryFromStdEnvVarOkServiceSocketAddressError {
    MissingEnvVarName { env_var_name: EnvVarName },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TryFromStdEnvVarOkSrcPlaceTypeError {
    MissingEnvVarName { env_var_name: EnvVarName },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TryFromStdEnvVarOkStartingCheckLinkError {
    MissingEnvVarName { env_var_name: EnvVarName },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TryFromStdEnvVarOkTimezoneError {
    MissingEnvVarName { env_var_name: EnvVarName },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TryFromStdEnvVarOkTracingLevelError {
    MissingEnvVarName { env_var_name: EnvVarName },
}

pub trait TryFromStdEnvVarOk: Sized {
    type Error;

    fn try_from_std_env_var_ok(env_var_name: EnvVarName) -> Self;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CorsAllowOrigin(EnvVarName);

impl TryFromStdEnvVarOk for CorsAllowOrigin {
    type Error = TryFromEnvError;

    fn try_from_std_env_var_ok(env_var_name: EnvVarName) -> Self {
        Self(env_var_name)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DatabaseUrl(EnvVarName);

impl TryFromStdEnvVarOk for DatabaseUrl {
    type Error = TryFromEnvError;

    fn try_from_std_env_var_ok(env_var_name: EnvVarName) -> Self {
        Self(env_var_name)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EnableApiGitCommitCheck(BoolConfigValue);

impl TryFromStdEnvVarOk for EnableApiGitCommitCheck {
    type Error = TryFromEnvError;

    fn try_from_std_env_var_ok(_env_var_name: EnvVarName) -> Self {
        Self(BoolConfigValue::False)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaximumSizeOfHttpBodyInBytes(ConfigParseStatus);

impl TryFromStdEnvVarOk for MaximumSizeOfHttpBodyInBytes {
    type Error = TryFromEnvError;

    fn try_from_std_env_var_ok(_env_var_name: EnvVarName) -> Self {
        Self(ConfigParseStatus::Valid)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MongoUrl(EnvVarName);

impl TryFromStdEnvVarOk for MongoUrl {
    type Error = TryFromEnvError;

    fn try_from_std_env_var_ok(env_var_name: EnvVarName) -> Self {
        Self(env_var_name)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PgPoolMaxConnections(ConfigParseStatus);

impl TryFromStdEnvVarOk for PgPoolMaxConnections {
    type Error = TryFromEnvError;

    fn try_from_std_env_var_ok(_env_var_name: EnvVarName) -> Self {
        Self(ConfigParseStatus::Valid)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RedisUrl(EnvVarName);

impl TryFromStdEnvVarOk for RedisUrl {
    type Error = TryFromEnvError;

    fn try_from_std_env_var_ok(env_var_name: EnvVarName) -> Self {
        Self(env_var_name)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ServiceSocketAddress(EnvVarName);

impl TryFromStdEnvVarOk for ServiceSocketAddress {
    type Error = TryFromEnvError;

    fn try_from_std_env_var_ok(env_var_name: EnvVarName) -> Self {
        Self(env_var_name)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SrcPlaceType(types::SrcPlaceType);

impl TryFromStdEnvVarOk for SrcPlaceType {
    type Error = TryFromEnvError;

    fn try_from_std_env_var_ok(_env_var_name: EnvVarName) -> Self {
        Self(types::SrcPlaceType::default_value())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StartingCheckLink(EnvVarName);

impl TryFromStdEnvVarOk for StartingCheckLink {
    type Error = TryFromEnvError;

    fn try_from_std_env_var_ok(env_var_name: EnvVarName) -> Self {
        Self(env_var_name)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Timezone(ConfigParseStatus);

impl TryFromStdEnvVarOk for Timezone {
    type Error = TryFromEnvError;

    fn try_from_std_env_var_ok(_env_var_name: EnvVarName) -> Self {
        Self(ConfigParseStatus::Valid)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TracingLevel(types::TracingLevel);

impl TryFromStdEnvVarOk for TracingLevel {
    type Error = TryFromEnvError;

    fn try_from_std_env_var_ok(_env_var_name: EnvVarName) -> Self {
        Self(types::TracingLevel::default_value())
    }
}

impl From<TryFromStdEnvVarOkCorsAllowOriginError> for TryFromEnvError {
    fn from(error: TryFromStdEnvVarOkCorsAllowOriginError) -> Self {
        match error {
            TryFromStdEnvVarOkCorsAllowOriginError::MissingEnvVarName { env_var_name } => {
                Self::MissingEnvVarName { env_var_name }
            }
        }
    }
}
