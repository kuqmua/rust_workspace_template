#[path = "admin_access_token_ttl_seconds.rs"]
mod admin_access_token_ttl_seconds;
#[path = "admin_login_failure_limit.rs"]
mod admin_login_failure_limit;
#[path = "admin_password_hash_concurrency.rs"]
mod admin_password_hash_concurrency;
#[path = "admin_positive_u64_parsing_error.rs"]
mod admin_positive_u64_parsing_error;
#[path = "admin_positive_usize_parsing_error.rs"]
mod admin_positive_usize_parsing_error;
#[path = "admin_refresh_token_ttl_seconds.rs"]
mod admin_refresh_token_ttl_seconds;
#[path = "admin_session_limit.rs"]
mod admin_session_limit;
#[path = "admin_sign_in_rate_limit.rs"]
mod admin_sign_in_rate_limit;
#[path = "admin_token_audience.rs"]
mod admin_token_audience;
#[path = "admin_token_issuer.rs"]
mod admin_token_issuer;
#[path = "parse_admin_positive_u64.rs"]
mod parse_admin_positive_u64;
#[path = "parse_admin_token_text.rs"]
mod parse_admin_token_text;
#[path = "try_from_std_env_var_ok_admin_password_hash_concurrency_error.rs"]
mod try_from_std_env_var_ok_admin_password_hash_concurrency_error;
#[path = "try_from_std_env_var_ok_admin_positive_u64_error.rs"]
mod try_from_std_env_var_ok_admin_positive_u64_error;
#[path = "try_from_std_env_var_ok_admin_token_text_error.rs"]
mod try_from_std_env_var_ok_admin_token_text_error;

pub use admin_access_token_ttl_seconds::*;
pub use admin_login_failure_limit::*;
pub use admin_password_hash_concurrency::*;
pub use admin_positive_u64_parsing_error::*;
pub use admin_positive_usize_parsing_error::*;
pub use admin_refresh_token_ttl_seconds::*;
pub use admin_session_limit::*;
pub use admin_sign_in_rate_limit::*;
pub use admin_token_audience::*;
pub use admin_token_issuer::*;
pub use try_from_std_env_var_ok_admin_password_hash_concurrency_error::*;
pub use try_from_std_env_var_ok_admin_positive_u64_error::*;
pub use try_from_std_env_var_ok_admin_token_text_error::*;

#[cfg(test)]
mod tests {
    #[test]
    fn positive_values_and_token_text_preserve_validation() {
        let ttl = <super::AdminAccessTokenTtlSeconds as super::super::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
            super::super::StdEnvVarOk::try_from(String::from(constants_str::VALUE_1)).expect("f39b6c2a positive_values_and_token_text_preserve_validation invariant must hold"),
        )
        .expect("de4810af positive_values_and_token_text_preserve_validation invariant must hold");
        assert_eq!(ttl.0.0.get(), 1u64);
        let zero = <super::AdminAccessTokenTtlSeconds as super::super::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
            super::super::StdEnvVarOk::try_from(String::from(constants_str::VALUE_0)).expect("a48e903d positive_values_and_token_text_preserve_validation invariant must hold"),
        );
        assert!(matches!(
            zero,
            Err(super::TryFromStdEnvVarOkAdminPositiveU64Error::IsZero)
        ));
        let issuer =
            <super::AdminTokenIssuer as super::super::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                super::super::StdEnvVarOk::try_from(String::from(constants_str::VALUE_535C6F8E)).expect("01f2db8a positive_values_and_token_text_preserve_validation invariant must hold"),
            )
            .expect("80c5df37 positive_values_and_token_text_preserve_validation invariant must hold");
        assert_eq!(issuer.as_ref(), "issuer");
    }
}
