#[cfg(test)]
mod tests {
    #[test]
    fn test_content_security_policy_rejects_header_injection() {
        let _error = crate::http_content_security_policy::HttpContentSecurityPolicy::try_from(
            constants_str::VALUE_0E50D890.to_owned(),
        )
        .expect_err(constants_str::VALUE_1E8BE8A1);
    }
}
