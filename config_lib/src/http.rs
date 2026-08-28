#[path = "content_security_policy.rs"]
mod content_security_policy;
#[path = "content_security_policy_error.rs"]
mod content_security_policy_error;
#[path = "maximum_size_of_http_body_in_bytes.rs"]
mod maximum_size_of_http_body_in_bytes;
#[path = "maximum_size_of_http_body_in_bytes_try_from_usize_error.rs"]
mod maximum_size_of_http_body_in_bytes_try_from_usize_error;
#[path = "try_from_std_env_var_ok_maximum_size_of_http_body_in_bytes_error.rs"]
mod try_from_std_env_var_ok_maximum_size_of_http_body_in_bytes_error;

pub use content_security_policy::ContentSecurityPolicy;
pub use content_security_policy_error::ContentSecurityPolicyError;
pub use maximum_size_of_http_body_in_bytes::{
    MaximumSizeOfHttpBodyInBytes, MaximumSizeOfHttpBodyInBytesProvider,
};
pub use maximum_size_of_http_body_in_bytes_try_from_usize_error::MaximumSizeOfHttpBodyInBytesTryFromUsizeError;
pub use try_from_std_env_var_ok_maximum_size_of_http_body_in_bytes_error::TryFromStdEnvVarOkMaximumSizeOfHttpBodyInBytesError;

#[cfg(test)]
mod tests {
    #[test]
    fn http_limits_and_csp_validate_boundary_values() {
        let body_limit = <super::MaximumSizeOfHttpBodyInBytes as super::super::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
            super::super::StdEnvVarOk::try_from(String::from(constants_str::VALUE_1)).expect("42f6d81c http_limits_and_csp_validate_boundary_values invariant must hold"),
        )
        .expect("85a01fbd http_limits_and_csp_validate_boundary_values invariant must hold");
        assert_eq!(body_limit.0, constants_usize::ONE);
        assert!(matches!(
            super::ContentSecurityPolicy::try_from(String::from("\n")),
            Err(super::ContentSecurityPolicyError::Empty)
        ));
    }
}
