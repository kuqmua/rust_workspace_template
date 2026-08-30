#[cfg(test)]
mod tests {
    #[test]
    fn positive_values_and_token_text_preserve_validation() {
        let ttl = <crate::admin_access_token_ttl_seconds::AdminAccessTokenTtlSeconds as crate::try_from_std_env_var_ok::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
            crate::std_env_var_ok::StdEnvVarOk::try_from(String::from(constants_str::catalog::VALUE_1)).expect("f39b6c2a positive_values_and_token_text_preserve_validation invariant must hold"),
        )
        .expect("de4810af positive_values_and_token_text_preserve_validation invariant must hold");
        assert_eq!(ttl.get(), 1u64);
        let zero = <crate::admin_access_token_ttl_seconds::AdminAccessTokenTtlSeconds as crate::try_from_std_env_var_ok::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
            crate::std_env_var_ok::StdEnvVarOk::try_from(String::from(constants_str::catalog::VALUE_0)).expect("a48e903d positive_values_and_token_text_preserve_validation invariant must hold"),
        );
        assert!(matches!(
            zero,
            Err(crate::try_from_std_env_var_ok_admin_positive_u64_error::TryFromStdEnvVarOkAdminPositiveU64Error::IsZero)
        ));
        let issuer =
            <crate::admin_token_issuer::AdminTokenIssuer as crate::try_from_std_env_var_ok::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                crate::std_env_var_ok::StdEnvVarOk::try_from(String::from(constants_str::test_fixtures::VALUE_535C6F8E)).expect("01f2db8a positive_values_and_token_text_preserve_validation invariant must hold"),
            )
            .expect("80c5df37 positive_values_and_token_text_preserve_validation invariant must hold");
        assert_eq!(issuer.as_ref(), "issuer");
    }
}
