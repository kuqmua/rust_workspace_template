#[cfg(test)]
mod tests {
    #[test]
    fn test_content_security_policy_header_exact_size_limits() {
        assert!([0usize, 1usize, 4095usize, 4096usize, 4097usize].into_iter().all(|length| {
            let text = constants_str::X.repeat(length);
            let result = crate::http_content_security_policy::HttpContentSecurityPolicy::try_from(text.clone());
            if length <= 4096usize {
                result.is_ok_and(|http_content_security_policy| http_content_security_policy.into_inner().as_bytes() == text.as_bytes())
            } else {
                matches!(result, Err(crate::http_content_security_policy_error::HttpContentSecurityPolicyError::InvalidHeaderValue))
            }
        }));
    }

    #[test]
    fn test_content_security_policy_rejects_header_injection() {
        let _error = crate::http_content_security_policy::HttpContentSecurityPolicy::try_from(
            constants_str::VALUE_0E50D890.to_owned(),
        )
        .expect_err(constants_str::VALUE_1E8BE8A1);
    }
}
