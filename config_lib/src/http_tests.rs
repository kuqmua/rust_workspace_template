#[cfg(test)]
mod tests {
    #[test]
    fn http_limits_and_csp_validate_boundary_values() {
        let body_limit = <crate::maximum_size_of_http_body_in_bytes::MaximumSizeOfHttpBodyInBytes as crate::try_from_std_env_var_ok::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
            crate::std_env_var_ok::StdEnvVarOk::try_from(String::from(constants_str::catalog::VALUE_1)).expect("42f6d81c http_limits_and_csp_validate_boundary_values invariant must hold"),
        )
        .expect("85a01fbd http_limits_and_csp_validate_boundary_values invariant must hold");
        assert_eq!(*body_limit, constants_usize::ONE);
        assert!(matches!(
            crate::content_security_policy::ContentSecurityPolicy::try_from(String::from("\n")),
            Err(crate::content_security_policy_error::ContentSecurityPolicyError::Empty)
        ));
    }
}
