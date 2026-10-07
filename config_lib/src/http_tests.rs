#[cfg(test)]
mod tests {
    #[test]
    fn test_http_limits_and_csp_validate_boundary_values() {
        let body_limit = <crate::maximum_size_of_http_body_in_bytes::MaximumSizeOfHttpBodyInBytes as crate::try_from_std_env_var_ok::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
            crate::std_env_var_ok::StdEnvVarOk::try_from(String::from(constants_str::VALUE_1)).expect(constants_str::DIAGNOSTIC_42F6D81C),
        )
        .expect(constants_str::DIAGNOSTIC_85A01FBD);
        assert_eq!(*body_limit, constants_usize::ONE);
        assert!(matches!(
            crate::content_security_policy::ContentSecurityPolicy::try_from(String::from(
                constants_str::NEWLINE
            )),
            Err(crate::content_security_policy_error::ContentSecurityPolicyError::Empty)
        ));
    }
    #[test]
    fn test_http_body_size_conversion_preserves_native_bounds_and_zero_error() {
        [1usize, std::num::NonZeroUsize::MAX.get()].into_iter().fold((), |(), value| {
            assert!(crate::maximum_size_of_http_body_in_bytes::MaximumSizeOfHttpBodyInBytes::try_from(value).is_ok_and(|maximum| *maximum == value));
            assert!(crate::std_env_var_ok::StdEnvVarOk::try_from(value.to_string()).is_ok_and(|std_env_var_ok| {
                <crate::maximum_size_of_http_body_in_bytes::MaximumSizeOfHttpBodyInBytes as crate::try_from_std_env_var_ok::TryFromStdEnvVarOk>::try_from_std_env_var_ok(std_env_var_ok).is_ok_and(|maximum| *maximum == value)
            }));
        });
        assert!(crate::maximum_size_of_http_body_in_bytes::MaximumSizeOfHttpBodyInBytes::try_from(0usize).is_err_and(|error| error == crate::maximum_size_of_http_body_in_bytes_try_from_usize_error::MaximumSizeOfHttpBodyInBytesTryFromUsizeError::IsZero));
        assert!(crate::std_env_var_ok::StdEnvVarOk::try_from(constants_str::VALUE_0.to_owned()).is_ok_and(|std_env_var_ok| {
            <crate::maximum_size_of_http_body_in_bytes::MaximumSizeOfHttpBodyInBytes as crate::try_from_std_env_var_ok::TryFromStdEnvVarOk>::try_from_std_env_var_ok(std_env_var_ok).is_err_and(|error| matches!(error, crate::try_from_std_env_var_ok_maximum_size_of_http_body_in_bytes_error::TryFromStdEnvVarOkMaximumSizeOfHttpBodyInBytesError::MaximumSizeOfHttpBodyInBytes { maximum_size_of_http_body_in_bytes } if maximum_size_of_http_body_in_bytes == crate::maximum_size_of_http_body_in_bytes_try_from_usize_error::MaximumSizeOfHttpBodyInBytesTryFromUsizeError::IsZero))
        }));
    }

    #[test]
    fn test_http_body_size_parse_errors_preserve_native_diagnostics() {
        let mut overflow = std::num::NonZeroUsize::MAX.to_string();
        overflow.push('0');
        [constants_str::EMPTY.to_owned(), constants_str::X.to_owned(), ['-', '1'].into_iter().collect::<String>(), [' ', '1'].into_iter().collect::<String>(), overflow]
            .into_iter().fold((), |(), text| {
                let expected = text.parse::<usize>();
                assert!(expected.is_err());
                let Err(source) = expected else { return; };
                assert!(crate::std_env_var_ok::StdEnvVarOk::try_from(text).is_ok_and(|std_env_var_ok| {
                    <crate::maximum_size_of_http_body_in_bytes::MaximumSizeOfHttpBodyInBytes as crate::try_from_std_env_var_ok::TryFromStdEnvVarOk>::try_from_std_env_var_ok(std_env_var_ok).is_err_and(|error| {
                        let crate::try_from_std_env_var_ok_maximum_size_of_http_body_in_bytes_error::TryFromStdEnvVarOkMaximumSizeOfHttpBodyInBytesError::UsizeParsing { usize_parsing } = error else { return false; };
                        format!("{usize_parsing:?}") == format!("{source:?}")
                    })
                }));
            });
    }
}
