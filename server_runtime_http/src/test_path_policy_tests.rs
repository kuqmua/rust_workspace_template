#[cfg(test)]
mod tests {
    #[test]
    fn test_proxy_path_matches_only_segment_prefix() {
        let path = crate::http_proxy_path::HttpProxyPath::try_from(
            crate::http_proxy_path_ref::HttpProxyPathRef::from(
                constants_str::TEST_PROXY_USERS_PATH,
            ),
        )
        .expect(constants_str::DIAGNOSTIC_6E90CB42);
        assert!(bool::from(
            crate::proxy_path_matches_prefix::proxy_path_matches_prefix(
                &path,
                crate::http_allowed_path_prefix_ref::HttpAllowedPathPrefixRef::from(
                    constants_str::TEST_PROXY_PREFIX
                )
            )
        ));
    }
    #[test]
    fn test_proxy_path_rejects_encoded_traversal() {
        assert_eq!(
            crate::http_proxy_path::HttpProxyPath::try_from(
                crate::http_proxy_path_ref::HttpProxyPathRef::from(
                    constants_str::TEST_ENCODED_PATH_TRAVERSAL
                )
            ),
            Err(crate::http_proxy_path_error::HttpProxyPathError::ForbiddenSyntax)
        );
    }
    #[test]
    fn test_identifier_path_normalizes_numbers_and_uuid_v4() {
        let normalized = crate::normalize_identifier_path::normalize_identifier_path(
            crate::http_request_path_ref::HttpRequestPathRef::from(
                constants_str::TEST_DYNAMIC_IDENTIFIER_PATH,
            ),
        )
        .expect(constants_str::DIAGNOSTIC_A36C01E4);
        assert_eq!(
            normalized.as_ref(),
            constants_str::TEST_NORMALIZED_IDENTIFIER_PATH
        );
    }
}

// Root-owned module compatibility wrappers.
