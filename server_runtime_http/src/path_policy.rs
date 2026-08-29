#[cfg(test)]
mod tests {
    #[test]
    fn proxy_path_matches_only_segment_prefix() {
        let path = crate::http_proxy_path::HttpProxyPath::try_from(
            crate::http_proxy_path_ref::HttpProxyPathRef::from(
                constants_str::test_fixtures::TEST_PROXY_USERS_PATH,
            ),
        )
        .expect("6e90cb42 proxy_path_matches_only_segment_prefix invariant must hold");
        assert!(bool::from(
            crate::proxy_path_matches_prefix::proxy_path_matches_prefix(
                &path,
                crate::http_allowed_path_prefix_ref::HttpAllowedPathPrefixRef::from(
                    constants_str::test_fixtures::TEST_PROXY_PREFIX
                )
            )
        ));
    }
    #[test]
    fn proxy_path_rejects_encoded_traversal() {
        assert_eq!(
            crate::http_proxy_path::HttpProxyPath::try_from(
                crate::http_proxy_path_ref::HttpProxyPathRef::from(
                    constants_str::test_fixtures::TEST_ENCODED_PATH_TRAVERSAL
                )
            ),
            Err(crate::http_proxy_path_error::HttpProxyPathError::ForbiddenSyntax)
        );
    }
    #[test]
    fn identifier_path_normalizes_numbers_and_uuid_v4() {
        let normalized = crate::normalize_identifier_path::normalize_identifier_path(
            crate::http_request_path_ref::HttpRequestPathRef::from(
                constants_str::catalog::TEST_DYNAMIC_IDENTIFIER_PATH,
            ),
        )
        .expect("a36c01e4 identifier_path_normalizes_numbers_and_uuid_v4 invariant must hold");
        assert_eq!(
            normalized.as_ref(),
            constants_str::test_fixtures::TEST_NORMALIZED_IDENTIFIER_PATH
        );
    }
}

// Root-owned module compatibility wrappers.
mod http_allowed_path_prefix_ref {}
mod http_normalized_path {}
mod http_normalized_path_error {}
mod http_proxy_path {}
mod http_proxy_path_error {}
mod http_proxy_path_prefix_match {}
mod http_proxy_path_ref {}
mod http_request_path_ref {}
mod normalize_identifier_path {}
mod proxy_path_matches_prefix {}
