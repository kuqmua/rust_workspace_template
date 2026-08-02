#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "path policy types stay grouped with their validation operations"
)]
#[derive(optml::Optml, Clone, Copy, Debug, newtype::FromInner)]
pub struct HttpProxyPathRef<'value_lt>(&'value_lt str);

#[derive(optml::Optml, Clone, Debug, Eq, PartialEq, newtype::AsRefStr)]
pub struct HttpProxyPath(String);
#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum HttpProxyPathError {
    #[error("proxy path must not be empty")]
    Empty,
    #[error("proxy path contains forbidden segment")]
    ForbiddenSegment,
    #[error("proxy path contains forbidden syntax")]
    ForbiddenSyntax,
}
impl TryFrom<String> for HttpProxyPath {
    type Error = HttpProxyPathError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > 8192usize {
            return Err(Self::Error::ForbiddenSyntax);
        }
        Self::try_from(HttpProxyPathRef::from(value.as_str()))
    }
}
impl TryFrom<HttpProxyPathRef<'_>> for HttpProxyPath {
    type Error = HttpProxyPathError;
    fn try_from(value: HttpProxyPathRef<'_>) -> Result<Self, Self::Error> {
        let path = value.0.trim().trim_start_matches('/');
        if path.is_empty() {
            return Err(Self::Error::Empty);
        }
        if path.len() > 8192usize {
            return Err(Self::Error::ForbiddenSyntax);
        }
        let lowercase = path.to_ascii_lowercase();
        if lowercase.starts_with(str_constants::HTTP_SCHEME_PREFIX)
            || lowercase.starts_with(str_constants::HTTPS_SCHEME_PREFIX)
            || [
                str_constants::ENCODED_DOT,
                str_constants::ENCODED_SLASH,
                str_constants::ENCODED_QUERY,
                str_constants::ENCODED_FRAGMENT,
                str_constants::ENCODED_BACKSLASH,
            ]
            .into_iter()
            .any(|encoded_syntax| lowercase.contains(encoded_syntax))
            || path.contains(['\\', '?', '#', '\0'])
            || path.chars().any(char::is_whitespace)
        {
            return Err(Self::Error::ForbiddenSyntax);
        }
        if path.split('/').any(|segment| {
            segment.is_empty()
                || segment == str_constants::CURRENT_PATH_SEGMENT
                || segment == str_constants::PARENT_PATH_SEGMENT
        }) {
            return Err(Self::Error::ForbiddenSegment);
        }
        Ok(Self(path.to_owned()))
    }
}

#[derive(optml::Optml, Clone, Copy, Debug, newtype::FromInner)]
pub struct HttpAllowedPathPrefixRef<'value_lt>(&'value_lt str);

#[derive(
    optml::Optml, Clone, Copy, Debug, Eq, PartialEq, newtype::FromInner, newtype::IntoInnerFrom,
)]
pub struct HttpProxyPathPrefixMatch(bool);
#[must_use]
pub fn proxy_path_matches_prefix(
    path: &HttpProxyPath,
    prefix: HttpAllowedPathPrefixRef<'_>,
) -> HttpProxyPathPrefixMatch {
    HttpProxyPathPrefixMatch::from(
        path.as_ref() == prefix.0
            || path
                .as_ref()
                .strip_prefix(prefix.0)
                .is_some_and(|suffix| suffix.starts_with('/')),
    )
}

#[derive(optml::Optml, Clone, Copy, Debug, newtype::FromInner)]
pub struct HttpRequestPathRef<'value_lt>(&'value_lt str);

#[derive(optml::Optml, Clone, Debug, Eq, PartialEq, newtype::AsRefStr)]
pub struct HttpNormalizedPath(String);
#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("normalized HTTP path is too long")]
pub struct HttpNormalizedPathError;
impl TryFrom<String> for HttpNormalizedPath {
    type Error = HttpNormalizedPathError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > 8192usize {
            Err(HttpNormalizedPathError)
        } else {
            Ok(Self(value))
        }
    }
}
#[must_use]
pub fn normalize_identifier_path(path: HttpRequestPathRef<'_>) -> Option<HttpNormalizedPath> {
    if path.0.len() > 8192usize || !path.0.bytes().any(|byte| byte.is_ascii_digit()) {
        return None;
    }
    let normalized = path
        .0
        .split('/')
        .map(|segment| {
            if !segment.is_empty()
                && segment.len() <= 19usize
                && segment.bytes().all(|byte| byte.is_ascii_digit())
            {
                str_constants::HTTP_NORMALIZED_IDENTIFIER_SEGMENT
            } else if uuid::Uuid::parse_str(segment)
                .is_ok_and(|value| value.get_version_num() == 4usize)
            {
                str_constants::HTTP_NORMALIZED_UUID_SEGMENT
            } else {
                segment
            }
        })
        .collect::<Vec<_>>()
        .join(str_constants::SLASH);
    if normalized == path.0 {
        None
    } else {
        HttpNormalizedPath::try_from(normalized).ok()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn proxy_path_matches_only_segment_prefix() {
        let path = super::HttpProxyPath::try_from(super::HttpProxyPathRef::from(
            str_constants::TEST_PROXY_USERS_PATH,
        ))
        .expect("6e90cb42");
        assert!(bool::from(super::proxy_path_matches_prefix(
            &path,
            super::HttpAllowedPathPrefixRef::from(str_constants::TEST_PROXY_PREFIX)
        )));
    }
    #[test]
    fn proxy_path_rejects_encoded_traversal() {
        assert_eq!(
            super::HttpProxyPath::try_from(super::HttpProxyPathRef::from(
                str_constants::TEST_ENCODED_PATH_TRAVERSAL
            )),
            Err(super::HttpProxyPathError::ForbiddenSyntax)
        );
    }
    #[test]
    fn identifier_path_normalizes_numbers_and_uuid_v4() {
        let normalized = super::normalize_identifier_path(super::HttpRequestPathRef::from(
            str_constants::TEST_DYNAMIC_IDENTIFIER_PATH,
        ))
        .expect("a36c01e4");
        assert_eq!(
            normalized.as_ref(),
            str_constants::TEST_NORMALIZED_IDENTIFIER_PATH
        );
    }
}
