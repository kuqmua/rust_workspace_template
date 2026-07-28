#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "HTTP policy types stay grouped with their corresponding resolver functions"
)]
#[derive(Clone, Copy, Debug, newtype::FromInner)]
pub struct HttpAuthorizationHeaderTextRef<'value_lt>(Option<&'value_lt str>);

#[derive(Clone, Copy, Eq, PartialEq, newtype::AsRefInner, newtype::FromInner)]
pub struct HttpBearerTokenRef<'value_lt>(&'value_lt str);
impl std::fmt::Debug for HttpBearerTokenRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(str_constants::REDACTED_ALT_3)
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BearerAuthorizationResolution<'value_lt> {
    Invalid,
    Missing,
    Resolved(HttpBearerTokenRef<'value_lt>),
}
#[must_use]
pub fn resolve_bearer_authorization(
    header: HttpAuthorizationHeaderTextRef<'_>,
) -> BearerAuthorizationResolution<'_> {
    let Some(value) = header.0 else {
        return BearerAuthorizationResolution::Missing;
    };
    if value.len() > 4096usize {
        return BearerAuthorizationResolution::Invalid;
    }
    let Some((scheme, token)) = value.split_once(' ') else {
        return BearerAuthorizationResolution::Invalid;
    };
    if !scheme.eq_ignore_ascii_case(str_constants::BEARER)
        || token.is_empty()
        || token.contains(char::is_whitespace)
    {
        BearerAuthorizationResolution::Invalid
    } else {
        BearerAuthorizationResolution::Resolved(HttpBearerTokenRef::from(token))
    }
}

#[derive(Clone, Copy, Debug, newtype::FromInner)]
pub struct HttpCookieHeadersRef<'value_lt>(&'value_lt http::HeaderMap);

#[derive(Clone, Copy, Debug, newtype::FromInner)]
pub struct HttpCookieNameRef<'value_lt>(&'value_lt str);

#[derive(
    Clone, Copy, Eq, PartialEq, newtype::AsRefInner, newtype::FromInner, newtype::IntoInnerFrom,
)]
pub struct HttpCookieValueRef<'value_lt>(&'value_lt str);
impl std::fmt::Debug for HttpCookieValueRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(str_constants::REDACTED_ALT_3)
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CookieResolution<'value_lt> {
    Invalid,
    Missing,
    Resolved(HttpCookieValueRef<'value_lt>),
}
#[must_use]
pub fn resolve_unique_cookie<'value_lt>(
    headers: HttpCookieHeadersRef<'value_lt>,
    name: HttpCookieNameRef<'_>,
) -> CookieResolution<'value_lt> {
    let mut header_values = headers.0.get_all(http::header::COOKIE).iter();
    let Some(header) = header_values.next() else {
        return CookieResolution::Missing;
    };
    if header_values.next().is_some() {
        return CookieResolution::Invalid;
    }
    let Ok(text) = header.to_str() else {
        return CookieResolution::Invalid;
    };
    if text.len() > 4096usize {
        return CookieResolution::Invalid;
    }
    match text
        .split(';')
        .try_fold((0usize, None), |(pair_count, found), pair| {
            if pair_count == 128usize {
                return std::ops::ControlFlow::Break(());
            }
            let Some((pair_name, value)) = pair.trim().split_once('=') else {
                return std::ops::ControlFlow::Continue((pair_count.saturating_add(1usize), found));
            };
            if pair_name != name.0 {
                return std::ops::ControlFlow::Continue((pair_count.saturating_add(1usize), found));
            }
            if found.is_some() {
                std::ops::ControlFlow::Break(())
            } else {
                std::ops::ControlFlow::Continue((pair_count.saturating_add(1usize), Some(value)))
            }
        }) {
        std::ops::ControlFlow::Break(()) => CookieResolution::Invalid,
        std::ops::ControlFlow::Continue((_pair_count, Some(value))) => {
            CookieResolution::Resolved(HttpCookieValueRef::from(value))
        }
        std::ops::ControlFlow::Continue((_pair_count, None)) => CookieResolution::Missing,
    }
}

#[derive(Clone, Copy, Debug, newtype::FromInner)]
pub struct HttpContentTypeTextRef<'value_lt>(Option<&'value_lt str>);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OptionalJsonContentType {
    ApplicationJson,
    Missing,
    NonJson,
}
#[must_use]
pub fn classify_optional_json_content_type(
    value: HttpContentTypeTextRef<'_>,
) -> OptionalJsonContentType {
    let Some(text) = value.0.map(str::trim).filter(|text| !text.is_empty()) else {
        return OptionalJsonContentType::Missing;
    };
    if text.len() > 4096usize {
        return OptionalJsonContentType::NonJson;
    }
    if text.split(';').next().is_some_and(|media_type| {
        media_type
            .trim()
            .eq_ignore_ascii_case(str_constants::APPLICATION_JSON)
    }) {
        OptionalJsonContentType::ApplicationJson
    } else {
        OptionalJsonContentType::NonJson
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OptionalJsonBodyPresence {
    Empty,
    NonEmpty,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OptionalJsonContentTypeDecision {
    Accept,
    RejectUnsupportedMediaType,
}
#[must_use]
pub const fn optional_json_content_type_decision(
    body: OptionalJsonBodyPresence,
    content_type: OptionalJsonContentType,
) -> OptionalJsonContentTypeDecision {
    match (body, content_type) {
        (_, OptionalJsonContentType::ApplicationJson)
        | (OptionalJsonBodyPresence::Empty, OptionalJsonContentType::Missing) => {
            OptionalJsonContentTypeDecision::Accept
        }
        _ => OptionalJsonContentTypeDecision::RejectUnsupportedMediaType,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn bearer_authorization_requires_exact_scheme_and_token() {
        assert!(matches!(
            super::resolve_bearer_authorization(super::HttpAuthorizationHeaderTextRef::from(Some(
                str_constants::TEST_BEARER_AUTHORIZATION
            ))),
            super::BearerAuthorizationResolution::Resolved(_)
        ));
        let secret = str_constants::NEVER_PRINT_THIS_VALUE;
        assert!(!format!("{:?}", super::HttpBearerTokenRef::from(secret)).contains(secret));
        assert!(!format!("{:?}", super::HttpCookieValueRef::from(secret)).contains(secret));
    }
    #[test]
    fn duplicate_cookie_is_invalid() {
        let mut headers = http::HeaderMap::new();
        let _previous = headers.insert(
            http::header::COOKIE,
            http::HeaderValue::from_static(str_constants::TEST_DUPLICATE_COOKIE),
        );
        assert_eq!(
            super::resolve_unique_cookie(
                super::HttpCookieHeadersRef::from(&headers),
                super::HttpCookieNameRef::from(str_constants::TEST_COOKIE_NAME)
            ),
            super::CookieResolution::Invalid
        );
    }
    #[test]
    fn json_content_type_supports_charset() {
        assert_eq!(
            super::classify_optional_json_content_type(super::HttpContentTypeTextRef::from(Some(
                str_constants::TEST_JSON_CONTENT_TYPE_WITH_CHARSET
            ))),
            super::OptionalJsonContentType::ApplicationJson
        );
    }
    #[test]
    fn optional_json_rejects_non_json_non_empty_body() {
        assert_eq!(
            super::optional_json_content_type_decision(
                super::OptionalJsonBodyPresence::NonEmpty,
                super::OptionalJsonContentType::NonJson
            ),
            super::OptionalJsonContentTypeDecision::RejectUnsupportedMediaType
        );
    }
}
