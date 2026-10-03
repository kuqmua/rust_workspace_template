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
    #[test]
    fn test_token_environment_conversions_check_empty_and_inclusive_length_bounds() {
        let parse_token_lengths = |std_env_var_ok: crate::std_env_var_ok::StdEnvVarOk| {
            [
                <crate::admin_token_issuer::AdminTokenIssuer as crate::try_from_std_env_var_ok::TryFromStdEnvVarOk>::try_from_std_env_var_ok(std_env_var_ok.clone()).map(|issuer| issuer.as_bounded_string().as_ref().len()),
                <crate::admin_token_audience::AdminTokenAudience as crate::try_from_std_env_var_ok::TryFromStdEnvVarOk>::try_from_std_env_var_ok(std_env_var_ok).map(|audience| audience.as_bounded_string().as_ref().len()),
            ]
        };
        [
            (constants_str::EMPTY.to_owned(), Err(crate::try_from_std_env_var_ok_admin_token_text_error::TryFromStdEnvVarOkAdminTokenTextError::Empty)),
            (constants_str::SPACE.to_owned(), Ok(1usize)),
            (constants_str::X.repeat(256usize), Ok(256usize)),
            (constants_str::X.repeat(257usize), Err(crate::try_from_std_env_var_ok_admin_token_text_error::TryFromStdEnvVarOkAdminTokenTextError::TooLong)),
        ]
        .into_iter()
        .fold((), |(), (value, expected)| {
            assert!(crate::std_env_var_ok::StdEnvVarOk::try_from(value).is_ok_and(|std_env_var_ok| {
                parse_token_lengths(std_env_var_ok).into_iter().all(|actual| actual == expected)
            }));
        });
    }
    #[test]
    fn test_positive_u64_administrator_settings_preserve_values_and_error_categories() {
        let parse_settings = |std_env_var_ok: crate::std_env_var_ok::StdEnvVarOk| {
            [
                <crate::admin_access_token_ttl_seconds::AdminAccessTokenTtlSeconds as crate::try_from_std_env_var_ok::TryFromStdEnvVarOk>::try_from_std_env_var_ok(std_env_var_ok.clone()).map(|positive| (*positive).get()),
                <crate::admin_refresh_token_ttl_seconds::AdminRefreshTokenTtlSeconds as crate::try_from_std_env_var_ok::TryFromStdEnvVarOk>::try_from_std_env_var_ok(std_env_var_ok.clone()).map(|positive| (*positive).get()),
                <crate::admin_sign_in_rate_limit::AdminSignInRateLimit as crate::try_from_std_env_var_ok::TryFromStdEnvVarOk>::try_from_std_env_var_ok(std_env_var_ok.clone()).map(|positive| (*positive).get()),
                <crate::admin_login_failure_limit::AdminLoginFailureLimit as crate::try_from_std_env_var_ok::TryFromStdEnvVarOk>::try_from_std_env_var_ok(std_env_var_ok).map(|positive| (*positive).get()),
            ]
        };
        [
            (constants_str::VALUE_1.to_owned(), Some(1u64), false),
            (u64::MAX.to_string(), Some(u64::MAX), false),
            (constants_str::VALUE_0.to_owned(), None, true),
            (constants_str::VALUE_F1234D75.to_owned(), None, false),
            (constants_str::EMPTY.to_owned(), None, false),
            ((u128::from(u64::MAX) + 1u128).to_string(), None, false),
        ]
        .into_iter()
        .fold((), |(), (value, expected, zero)| {
            assert!(crate::std_env_var_ok::StdEnvVarOk::try_from(value).is_ok_and(|std_env_var_ok| {
                parse_settings(std_env_var_ok).into_iter().all(|actual| match expected {
                    Some(positive) => actual.is_ok_and(|parsed| parsed == positive),
                    None if zero => matches!(actual, Err(crate::try_from_std_env_var_ok_admin_positive_u64_error::TryFromStdEnvVarOkAdminPositiveU64Error::IsZero)),
                    None => matches!(actual, Err(crate::try_from_std_env_var_ok_admin_positive_u64_error::TryFromStdEnvVarOkAdminPositiveU64Error::Parse { .. })),
                })
            }));
        });
    }
    #[test]
    fn test_administrator_session_limit_validates_positive_and_invalid_input() {
        [
            (constants_str::VALUE_1, Some(1usize), false),
            (constants_str::VALUE_0, None, true),
            (constants_str::VALUE_F1234D75, None, false),
        ]
        .into_iter()
        .fold((), |(), (value, expected, zero)| {
            assert!(crate::std_env_var_ok::StdEnvVarOk::try_from(value.to_owned()).is_ok_and(|std_env_var_ok| {
                let parsed = <crate::admin_session_limit::AdminSessionLimit as crate::try_from_std_env_var_ok::TryFromStdEnvVarOk>::try_from_std_env_var_ok(std_env_var_ok);
                match expected {
                    Some(positive) => parsed.is_ok_and(|limit| (*limit).get() == positive),
                    None if zero => matches!(parsed, Err(crate::try_from_std_env_var_ok_admin_positive_u64_error::TryFromStdEnvVarOkAdminPositiveU64Error::IsZero)),
                    None => matches!(parsed, Err(crate::try_from_std_env_var_ok_admin_positive_u64_error::TryFromStdEnvVarOkAdminPositiveU64Error::Parse { .. })),
                }
            }));
        });
    }
}
