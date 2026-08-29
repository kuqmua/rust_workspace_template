#[cfg(test)]
mod tests {
    #[test]
    fn content_security_policy_rejects_header_injection() {
        let _error = crate::http_content_security_policy::HttpContentSecurityPolicy::try_from(
            constants_str::test_fixtures::VALUE_0E50D890.to_owned(),
        )
        .expect_err(constants_str::test_fixtures::VALUE_1E8BE8A1);
    }
}

// Root-owned module compatibility wrappers.
mod forwarded_proto_trust {}
mod http_content_security_policy {}
mod http_content_security_policy_error {}
mod security_headers_layer {}
mod security_headers_service {}
mod security_headers_tower_layer {}
