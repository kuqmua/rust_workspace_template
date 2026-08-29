#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq, newtype::AsRefStr,
)]
pub struct HttpProxyPath(String);

impl TryFrom<String> for HttpProxyPath {
    type Error = crate::http_proxy_path_error::HttpProxyPathError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > constants_usize::VALUE_8_192 {
            return Err(Self::Error::ForbiddenSyntax);
        }
        Self::try_from(crate::http_proxy_path_ref::HttpProxyPathRef::from(
            value.as_str(),
        ))
    }
}

impl TryFrom<crate::http_proxy_path_ref::HttpProxyPathRef<'_>> for HttpProxyPath {
    type Error = crate::http_proxy_path_error::HttpProxyPathError;

    fn try_from(
        value: crate::http_proxy_path_ref::HttpProxyPathRef<'_>,
    ) -> Result<Self, Self::Error> {
        let path = value.0.trim().trim_start_matches('/');
        if path.is_empty() {
            return Err(Self::Error::Empty);
        }
        if path.len() > constants_usize::VALUE_8_192 {
            return Err(Self::Error::ForbiddenSyntax);
        }
        let starts_with_ignore_ascii_case = |prefix: &str| {
            path.as_bytes()
                .get(..prefix.len())
                .is_some_and(|prefix_bytes| prefix_bytes.eq_ignore_ascii_case(prefix.as_bytes()))
        };
        let contains_ignore_ascii_case = |pattern: &str| {
            path.as_bytes()
                .windows(pattern.len())
                .any(|window| window.eq_ignore_ascii_case(pattern.as_bytes()))
        };
        if starts_with_ignore_ascii_case(constants_str::HTTP_SCHEME_PREFIX)
            || starts_with_ignore_ascii_case(constants_str::HTTPS_SCHEME_PREFIX)
            || [
                constants_str::ENCODED_DOT,
                constants_str::ENCODED_SLASH,
                constants_str::ENCODED_QUERY,
                constants_str::test_fixtures::ENCODED_FRAGMENT,
                constants_str::test_fixtures::ENCODED_BACKSLASH,
            ]
            .into_iter()
            .any(contains_ignore_ascii_case)
            || path.contains(['\\', '?', '#', '\0'])
            || path.chars().any(char::is_whitespace)
        {
            return Err(Self::Error::ForbiddenSyntax);
        }
        if path.split('/').any(|segment| {
            segment.is_empty()
                || segment == constants_str::test_fixtures::CURRENT_PATH_SEGMENT
                || segment == constants_str::test_fixtures::PARENT_PATH_SEGMENT
        }) {
            return Err(Self::Error::ForbiddenSegment);
        }
        Ok(Self(path.to_owned()))
    }
}
