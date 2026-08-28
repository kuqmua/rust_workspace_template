// The owner module retains lint-sensitive semantics from the original implementation.
#![allow(clippy::arbitrary_source_item_ordering)] // configuration declarations stay grouped with their parse errors and TryFromStdEnvVarOk implementations
#[path = "admin.rs"]
mod admin;
#[path = "admin_jwt.rs"]
mod admin_jwt;
#[path = "bool_flags.rs"]
mod bool_flags;
#[path = "http.rs"]
mod http;
#[path = "pg_pool.rs"]
mod pg_pool;
#[path = "types.rs"]
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
#[path = "std_env_var_ok.rs"]
mod std_env_var_ok;
pub use std_env_var_ok::*;
#[path = "config_lib_string_wrapper_try_from_string_error.rs"]
mod config_lib_string_wrapper_try_from_string_error;
pub use config_lib_string_wrapper_try_from_string_error::*;
#[path = "std_env_var_ok_ref.rs"]
mod std_env_var_ok_ref;
pub use std_env_var_ok_ref::*;
#[path = "env_var_name_ref.rs"]
mod env_var_name_ref;
pub use env_var_name_ref::*;
#[path = "env_var_name.rs"]
mod env_var_name;
pub use env_var_name::*;
#[path = "chrono_fixed_offset_error.rs"]
mod chrono_fixed_offset_error;
pub use chrono_fixed_offset_error::*;
#[path = "i32_parse_int_error.rs"]
mod i32_parse_int_error;
pub use i32_parse_int_error::*;
#[path = "u32_parse_int_error.rs"]
mod u32_parse_int_error;
pub use u32_parse_int_error::*;
#[path = "usize_parse_int_error.rs"]
mod usize_parse_int_error;
pub use usize_parse_int_error::*;
#[path = "timezone_seconds.rs"]
mod timezone_seconds;
use timezone_seconds::TimezoneSeconds;
#[path = "chrono_east_fixed_offset.rs"]
mod chrono_east_fixed_offset;
use chrono_east_fixed_offset::ChronoEastFixedOffset;
#[path = "try_from_std_env_var_ok.rs"]
mod try_from_std_env_var_ok;
pub use try_from_std_env_var_ok::*;
#[path = "config_field_sensitivity.rs"]
mod config_field_sensitivity;
pub use config_field_sensitivity::*;
#[path = "config_field_requirement.rs"]
mod config_field_requirement;
pub use config_field_requirement::*;
#[path = "config_example_validity.rs"]
mod config_example_validity;
pub use config_example_validity::*;
#[path = "config_field_example_ref.rs"]
mod config_field_example_ref;
pub use config_field_example_ref::*;
#[path = "config_rust_type_name.rs"]
mod config_rust_type_name;
pub use config_rust_type_name::*;
#[path = "config_field_descriptor.rs"]
mod config_field_descriptor;
pub use config_field_descriptor::*;
#[path = "std_config_secret_string.rs"]
mod std_config_secret_string;
pub use std_config_secret_string::*;
#[path = "secrecy_secret_box_string.rs"]
mod secrecy_secret_box_string;
pub use secrecy_secret_box_string::*;
#[path = "config_non_zero_u64.rs"]
mod config_non_zero_u64;
pub use config_non_zero_u64::*;
#[path = "config_non_zero_usize.rs"]
mod config_non_zero_usize;
pub use config_non_zero_usize::*;
#[path = "parse_int_error.rs"]
mod parse_int_error;
pub use parse_int_error::*;
#[path = "parse_bool_error.rs"]
mod parse_bool_error;
pub use parse_bool_error::*;
#[path = "chrono_timezone.rs"]
mod chrono_timezone;
pub use chrono_timezone::*;
#[path = "try_from_std_env_var_ok_timezone_error.rs"]
mod try_from_std_env_var_ok_timezone_error;
pub use try_from_std_env_var_ok_timezone_error::*;
#[path = "try_from_std_env_var_ok_svc_mode_error.rs"]
mod try_from_std_env_var_ok_svc_mode_error;
pub use try_from_std_env_var_ok_svc_mode_error::*;
#[path = "parse_required_env_var.rs"]
mod parse_required_env_var;
pub use parse_required_env_var::*;
#[path = "try_map_non_empty_env_value.rs"]
mod try_map_non_empty_env_value;
use try_map_non_empty_env_value::try_map_non_empty_env_value;
#[path = "parse_from_str_with_error.rs"]
mod parse_from_str_with_error;
use parse_from_str_with_error::parse_from_str_with_error;
#[path = "parse_east_fixed_offset.rs"]
mod parse_east_fixed_offset;
use parse_east_fixed_offset::parse_east_fixed_offset;
#[path = "config_lib_string_wrapper_max_len.rs"]
mod config_lib_string_wrapper_max_len;
use config_lib_string_wrapper_max_len::CONFIG_LIB_STRING_WRAPPER_MAX_LEN;

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

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
