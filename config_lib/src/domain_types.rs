// The owner module retains lint-sensitive semantics from the original implementation.
pub mod types {
    pub use super::super::types::*;
}

pub use super::admin::{
    AdminAccessTokenTtlSeconds, AdminLoginFailureLimit, AdminLoginFailureLimitProvider,
    AdminPasswordHashConcurrency, AdminRefreshTokenTtlSeconds, AdminSessionLimit,
    AdminSessionLimitProvider, AdminSignInRateLimit, AdminSignInRateLimitProvider,
    AdminTokenAudience, AdminTokenAudienceProvider, AdminTokenAudienceTryFromStringError,
    AdminTokenIssuer, AdminTokenIssuerProvider, AdminTokenIssuerTryFromStringError,
    TryFromStdEnvVarOkAdminPasswordHashConcurrencyError, TryFromStdEnvVarOkAdminPositiveU64Error,
    TryFromStdEnvVarOkAdminTokenTextError,
};
pub use super::admin_jwt::{
    AdminJwtSecret, AdminJwtSecretProvider, TryFromStdEnvVarOkAdminJwtSecretError,
};
pub use super::chrono_fixed_offset_error::*;
pub use super::chrono_timezone::*;
pub use super::config_example_validity::*;
pub use super::config_field_descriptor::*;
pub use super::config_field_example_ref::*;
pub use super::config_field_requirement::*;
pub use super::config_field_sensitivity::*;
use super::config_lib_string_wrapper_max_len::CONFIG_LIB_STRING_WRAPPER_MAX_LEN;
pub use super::config_lib_string_wrapper_try_from_string_error::*;
pub use super::config_rust_type_name::*;
pub use super::env_var_name::*;
pub use super::env_var_name_ref::*;
pub use super::http::{
    ContentSecurityPolicy, ContentSecurityPolicyError, MaximumSizeOfHttpBodyInBytes,
    MaximumSizeOfHttpBodyInBytesProvider, MaximumSizeOfHttpBodyInBytesTryFromUsizeError,
    TryFromStdEnvVarOkMaximumSizeOfHttpBodyInBytesError,
};
pub use super::i32_parse_int_error::*;
pub use super::parse_bool_error::*;
use super::parse_east_fixed_offset::parse_east_fixed_offset;
use super::parse_from_str_with_error::parse_from_str_with_error;
pub use super::parse_int_error::*;
pub use super::parse_required_env_var::*;
pub use super::pg_pool::{
    PgPoolAcquireTimeoutSeconds, PgPoolConfigParseError, PgPoolIdleTimeoutSeconds,
    PgPoolMaxConnections, PgPoolMaxConnectionsProvider, PgPoolMaxConnectionsTryFromU32Error,
    PgPoolMaxLifetimeSeconds, PgPoolMinConnections, RequestTimeoutSeconds,
    TryFromStdEnvVarOkPgPoolMaxConnectionsError,
};
pub use super::secrecy_secret_box_string::*;
pub use super::std_config_secret_string::*;
pub use super::std_env_var_ok::*;
pub use super::std_env_var_ok_ref::*;
use super::timezone_seconds::TimezoneSeconds;
pub use super::try_from_std_env_var_ok::*;
pub use super::try_from_std_env_var_ok_svc_mode_error::*;
pub use super::try_from_std_env_var_ok_timezone_error::*;
use super::try_map_non_empty_env_value::try_map_non_empty_env_value;
pub use super::u32_parse_int_error::*;
pub use super::usize_parse_int_error::*;
pub use super::{
    AdminCookieSecure, AdminCookieSecureProvider, AdminSwaggerEnabled, AdminSwaggerEnabledProvider,
    HttpGzipEnabled, ProductionMode,
};

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
config_lib_macros::impl_try_from_parse_string_error!(
    TracingLevel,
    TryFromStdEnvVarOkTracingLevelError,
    types::TracingLevel,
    AppStateTracingLevelParsing,
    app_state_tracing_type_parsing
);
