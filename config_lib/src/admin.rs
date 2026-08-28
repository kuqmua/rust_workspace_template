pub use crate::admin_access_token_ttl_seconds::*;
pub use crate::admin_login_failure_limit::*;
pub use crate::admin_password_hash_concurrency::*;
pub use crate::admin_refresh_token_ttl_seconds::*;
pub use crate::admin_session_limit::*;
pub use crate::admin_sign_in_rate_limit::*;
pub use crate::admin_token_audience::*;
pub use crate::admin_token_issuer::*;
pub use crate::try_from_std_env_var_ok_admin_password_hash_concurrency_error::*;
pub use crate::try_from_std_env_var_ok_admin_positive_u64_error::*;
pub use crate::try_from_std_env_var_ok_admin_token_text_error::*;

#[cfg(test)]
mod tests {
    #[test]
    fn positive_values_and_token_text_preserve_validation() {
        let ttl = <super::AdminAccessTokenTtlSeconds as crate::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
            crate::StdEnvVarOk::try_from(String::from(constants_str::VALUE_1)).expect("f39b6c2a positive_values_and_token_text_preserve_validation invariant must hold"),
        )
        .expect("de4810af positive_values_and_token_text_preserve_validation invariant must hold");
        assert_eq!(ttl.0.get(), 1u64);
        let zero = <super::AdminAccessTokenTtlSeconds as crate::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
            crate::StdEnvVarOk::try_from(String::from(constants_str::VALUE_0)).expect("a48e903d positive_values_and_token_text_preserve_validation invariant must hold"),
        );
        assert!(matches!(
            zero,
            Err(super::TryFromStdEnvVarOkAdminPositiveU64Error::IsZero)
        ));
        let issuer =
            <super::AdminTokenIssuer as crate::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                crate::StdEnvVarOk::try_from(String::from(constants_str::VALUE_535C6F8E)).expect("01f2db8a positive_values_and_token_text_preserve_validation invariant must hold"),
            )
            .expect("80c5df37 positive_values_and_token_text_preserve_validation invariant must hold");
        assert_eq!(issuer.as_ref(), "issuer");
    }
}
