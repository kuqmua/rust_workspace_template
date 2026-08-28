// The owner module retains lint-sensitive semantics from the original implementation.
#![allow(clippy::arbitrary_source_item_ordering)] // configuration declarations stay grouped with their parse errors and TryFromStdEnvVarOk implementations
pub mod types {
    pub use crate::types::*;
}

pub use crate::admin::{
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
pub use crate::admin_jwt::{
    AdminJwtSecret, AdminJwtSecretProvider, TryFromStdEnvVarOkAdminJwtSecretError,
};
pub use crate::bool_flags::{
    AdminBoolParsingError, AdminCookieSecure, AdminCookieSecureProvider, AdminSwaggerEnabled,
    AdminSwaggerEnabledProvider, HttpGzipEnabled, ProductionMode,
    TryFromStdEnvVarOkAdminCookieSecureError,
};
pub use crate::chrono_fixed_offset_error::*;
pub use crate::chrono_timezone::*;
pub use crate::config_example_validity::*;
pub use crate::config_field_descriptor::*;
pub use crate::config_field_example_ref::*;
pub use crate::config_field_requirement::*;
pub use crate::config_field_sensitivity::*;
use crate::config_lib_string_wrapper_max_len::CONFIG_LIB_STRING_WRAPPER_MAX_LEN;
pub use crate::config_lib_string_wrapper_try_from_string_error::*;
pub use crate::config_non_zero_u64::*;
pub use crate::config_non_zero_usize::*;
pub use crate::config_rust_type_name::*;
pub use crate::env_var_name::*;
pub use crate::env_var_name_ref::*;
pub use crate::http::{
    ContentSecurityPolicy, ContentSecurityPolicyError, MaximumSizeOfHttpBodyInBytes,
    MaximumSizeOfHttpBodyInBytesProvider, MaximumSizeOfHttpBodyInBytesTryFromUsizeError,
    TryFromStdEnvVarOkMaximumSizeOfHttpBodyInBytesError,
};
pub use crate::i32_parse_int_error::*;
pub use crate::parse_bool_error::*;
use crate::parse_east_fixed_offset::parse_east_fixed_offset;
use crate::parse_from_str_with_error::parse_from_str_with_error;
pub use crate::parse_int_error::*;
pub use crate::parse_required_env_var::*;
pub use crate::pg_pool::{
    PgPoolAcquireTimeoutSeconds, PgPoolConfigParseError, PgPoolIdleTimeoutSeconds,
    PgPoolMaxConnections, PgPoolMaxConnectionsProvider, PgPoolMaxConnectionsTryFromU32Error,
    PgPoolMaxLifetimeSeconds, PgPoolMinConnections, RequestTimeoutSeconds,
    TryFromStdEnvVarOkPgPoolMaxConnectionsError,
};
pub use crate::secrecy_secret_box_string::*;
pub use crate::std_config_secret_string::*;
pub use crate::std_env_var_ok::*;
pub use crate::std_env_var_ok_ref::*;
use crate::timezone_seconds::TimezoneSeconds;
pub use crate::try_from_std_env_var_ok::*;
pub use crate::try_from_std_env_var_ok_svc_mode_error::*;
pub use crate::try_from_std_env_var_ok_timezone_error::*;
use crate::try_map_non_empty_env_value::try_map_non_empty_env_value;
pub use crate::u32_parse_int_error::*;
pub use crate::usize_parse_int_error::*;

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
