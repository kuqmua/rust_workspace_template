#[cfg(test)]
mod tests {
    #[test]
    fn test_proxy_path_size_limits_preserve_owned_and_borrowed_normalization_rules() {
        assert!([8191usize, 8192usize, 8193usize].into_iter().all(|length| {
            let text = constants_str::X.repeat(length);
            let owned = crate::http_proxy_path::HttpProxyPath::try_from(text.clone());
            let borrowed = crate::http_proxy_path::HttpProxyPath::try_from(
                crate::http_proxy_path_ref::HttpProxyPathRef::from(text.as_str()),
            );
            [owned, borrowed].into_iter().all(|result| {
                if length <= 8192usize {
                    result.is_ok_and(|path| path.as_ref() == text)
                } else {
                    result == Err(crate::http_proxy_path_error::HttpProxyPathError::ForbiddenSyntax)
                }
            })
        }));
        let normalized = constants_str::X.repeat(8192usize);
        let mut padded = String::from('/');
        padded.push_str(&normalized);
        padded.push(' ');
        assert_eq!(
            crate::http_proxy_path::HttpProxyPath::try_from(padded.clone()),
            Err(crate::http_proxy_path_error::HttpProxyPathError::ForbiddenSyntax)
        );
        assert!(
            crate::http_proxy_path::HttpProxyPath::try_from(
                crate::http_proxy_path_ref::HttpProxyPathRef::from(padded.as_str())
            )
            .is_ok_and(|path| path.as_ref() == normalized)
        );
    }

    #[test]
    fn test_proxy_path_rejects_encoded_delimiters_and_invalid_segments() {
        assert!(
            [
                constants_str::ENCODED_DOT,
                constants_str::ENCODED_SLASH,
                constants_str::ENCODED_QUERY,
                constants_str::ENCODED_FRAGMENT,
                constants_str::ENCODED_BACKSLASH,
                constants_str::HTTP_SCHEME_PREFIX,
                constants_str::HTTPS_SCHEME_PREFIX,
            ]
            .into_iter()
            .all(|syntax| {
                [syntax.to_ascii_lowercase(), syntax.to_ascii_uppercase()]
                    .into_iter()
                    .all(|value| {
                        crate::http_proxy_path::HttpProxyPath::try_from(value)
                            == Err(
                                crate::http_proxy_path_error::HttpProxyPathError::ForbiddenSyntax,
                            )
                    })
            })
        );
        assert!(
            [
                constants_str::CURRENT_PATH_SEGMENT,
                constants_str::PARENT_PATH_SEGMENT
            ]
            .into_iter()
            .all(|segment| {
                let path = format!("{}/{}/{}", constants_str::X, segment, constants_str::X);
                crate::http_proxy_path::HttpProxyPath::try_from(path)
                    == Err(crate::http_proxy_path_error::HttpProxyPathError::ForbiddenSegment)
            })
        );
        assert_eq!(
            crate::http_proxy_path::HttpProxyPath::try_from('/'.to_string()),
            Err(crate::http_proxy_path_error::HttpProxyPathError::Empty)
        );
    }

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
