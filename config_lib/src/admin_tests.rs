#[cfg(test)]
mod tests {
    #[test]
    fn test_positive_values_and_token_text_preserve_validation() {
        let ttl = <crate::admin_access_token_ttl_seconds::AdminAccessTokenTtlSeconds as crate::try_from_std_env_var_ok::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
            crate::std_env_var_ok::StdEnvVarOk::try_from(String::from(constants_str::VALUE_1)).expect(constants_str::DIAGNOSTIC_F39B6C2A),
        )
        .expect(constants_str::DIAGNOSTIC_DE4810AF);
        assert_eq!(ttl.get(), 1u64);
        let zero = <crate::admin_access_token_ttl_seconds::AdminAccessTokenTtlSeconds as crate::try_from_std_env_var_ok::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
            crate::std_env_var_ok::StdEnvVarOk::try_from(String::from(constants_str::VALUE_0)).expect(constants_str::DIAGNOSTIC_A48E903D),
        );
        assert!(matches!(
            zero,
            Err(crate::try_from_std_env_var_ok_admin_positive_u64_error::TryFromStdEnvVarOkAdminPositiveU64Error::IsZero)
        ));
        let issuer =
            <crate::admin_token_issuer::AdminTokenIssuer as crate::try_from_std_env_var_ok::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                crate::std_env_var_ok::StdEnvVarOk::try_from(String::from(constants_str::VALUE_535C6F8E)).expect(constants_str::DIAGNOSTIC_01F2DB8A),
            )
            .expect(constants_str::DIAGNOSTIC_80C5DF37);
        assert_eq!(issuer.as_ref(), constants_str::VALUE_535C6F8E);
    }
}
