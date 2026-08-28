#[path = "http_allowed_path_prefix_ref.rs"]
mod http_allowed_path_prefix_ref;
#[path = "http_normalized_path.rs"]
mod http_normalized_path;
#[path = "http_normalized_path_error.rs"]
mod http_normalized_path_error;
#[path = "http_proxy_path.rs"]
mod http_proxy_path;
#[path = "http_proxy_path_error.rs"]
mod http_proxy_path_error;
#[path = "http_proxy_path_prefix_match.rs"]
mod http_proxy_path_prefix_match;
#[path = "http_proxy_path_ref.rs"]
mod http_proxy_path_ref;
#[path = "http_request_path_ref.rs"]
mod http_request_path_ref;
#[path = "normalize_identifier_path.rs"]
mod normalize_identifier_path;
#[path = "proxy_path_matches_prefix.rs"]
mod proxy_path_matches_prefix;

pub use http_allowed_path_prefix_ref::HttpAllowedPathPrefixRef;
pub use http_normalized_path::HttpNormalizedPath;
pub use http_normalized_path_error::HttpNormalizedPathError;
pub use http_proxy_path::HttpProxyPath;
pub use http_proxy_path_error::HttpProxyPathError;
pub use http_proxy_path_prefix_match::HttpProxyPathPrefixMatch;
pub use http_proxy_path_ref::HttpProxyPathRef;
pub use http_request_path_ref::HttpRequestPathRef;
pub use normalize_identifier_path::normalize_identifier_path;
pub use proxy_path_matches_prefix::proxy_path_matches_prefix;

#[cfg(test)]
mod tests {
    #[test]
    fn proxy_path_matches_only_segment_prefix() {
        let path = super::HttpProxyPath::try_from(super::HttpProxyPathRef::from(
            constants_str::TEST_PROXY_USERS_PATH,
        ))
        .expect("6e90cb42 proxy_path_matches_only_segment_prefix invariant must hold");
        assert!(bool::from(super::proxy_path_matches_prefix(
            &path,
            super::HttpAllowedPathPrefixRef::from(constants_str::TEST_PROXY_PREFIX)
        )));
    }
    #[test]
    fn proxy_path_rejects_encoded_traversal() {
        assert_eq!(
            super::HttpProxyPath::try_from(super::HttpProxyPathRef::from(
                constants_str::TEST_ENCODED_PATH_TRAVERSAL
            )),
            Err(super::HttpProxyPathError::ForbiddenSyntax)
        );
    }
    #[test]
    fn identifier_path_normalizes_numbers_and_uuid_v4() {
        let normalized = super::normalize_identifier_path(super::HttpRequestPathRef::from(
            constants_str::TEST_DYNAMIC_IDENTIFIER_PATH,
        ))
        .expect("a36c01e4 identifier_path_normalizes_numbers_and_uuid_v4 invariant must hold");
        assert_eq!(
            normalized.as_ref(),
            constants_str::TEST_NORMALIZED_IDENTIFIER_PATH
        );
    }
}
