pub use super::bearer_authorization_resolution::BearerAuthorizationResolution;
pub use super::classify_optional_json_content_type::classify_optional_json_content_type;
pub use super::cookie_resolution::CookieResolution;
pub use super::http_authorization_header_text_ref::HttpAuthorizationHeaderTextRef;
pub use super::http_bearer_token_ref::HttpBearerTokenRef;
pub use super::http_content_type_text_ref::HttpContentTypeTextRef;
pub use super::http_cookie_headers_ref::HttpCookieHeadersRef;
pub use super::http_cookie_name_ref::HttpCookieNameRef;
pub use super::http_cookie_value_ref::HttpCookieValueRef;
pub use super::optional_json_body_presence::OptionalJsonBodyPresence;
pub use super::optional_json_content_type::OptionalJsonContentType;
pub use super::optional_json_content_type_decision::OptionalJsonContentTypeDecision;
pub use super::resolve_bearer_authorization::resolve_bearer_authorization;
pub use super::resolve_optional_json_content_type_decision::resolve_optional_json_content_type_decision;
pub use super::resolve_unique_cookie::resolve_unique_cookie;
#[cfg(test)]
mod tests {
    #[test]
    fn bearer_authorization_requires_exact_scheme_and_token() {
        assert!(matches!(
            super::resolve_bearer_authorization(super::HttpAuthorizationHeaderTextRef::from(Some(
                constants_str::TEST_BEARER_AUTHORIZATION
            ))),
            super::BearerAuthorizationResolution::Resolved(_)
        ));
        let secret = constants_str::NEVER_PRINT_THIS_VALUE;
        assert!(!format!("{:?}", super::HttpBearerTokenRef::from(secret)).contains(secret));
        assert!(!format!("{:?}", super::HttpCookieValueRef::from(secret)).contains(secret));
    }
    #[test]
    fn duplicate_cookie_is_invalid() {
        let mut headers = http::HeaderMap::new();
        let _previous = headers.insert(
            http::header::COOKIE,
            http::HeaderValue::from_static(constants_str::TEST_DUPLICATE_COOKIE),
        );
        assert_eq!(
            super::resolve_unique_cookie(
                super::HttpCookieHeadersRef::from(&headers),
                super::HttpCookieNameRef::from(constants_str::TEST_COOKIE_NAME)
            ),
            super::CookieResolution::Invalid
        );
    }
    #[test]
    fn json_content_type_supports_charset() {
        assert_eq!(
            super::classify_optional_json_content_type(super::HttpContentTypeTextRef::from(Some(
                constants_str::TEST_JSON_CONTENT_TYPE_WITH_CHARSET
            ))),
            super::OptionalJsonContentType::ApplicationJson
        );
    }
    #[test]
    fn optional_json_rejects_non_json_non_empty_body() {
        assert_eq!(
            super::resolve_optional_json_content_type_decision(
                super::OptionalJsonBodyPresence::NonEmpty,
                super::OptionalJsonContentType::NonJson
            ),
            super::OptionalJsonContentTypeDecision::RejectUnsupportedMediaType
        );
    }
}

// Root-owned module compatibility wrappers.
mod bearer_authorization_resolution {
    pub use super::super::bearer_authorization_resolution::*;
}
mod classify_optional_json_content_type {
    pub use super::super::classify_optional_json_content_type::*;
}
mod cookie_resolution {
    pub use super::super::cookie_resolution::*;
}
mod http_authorization_header_text_ref {
    pub use super::super::http_authorization_header_text_ref::*;
}
mod http_bearer_token_ref {
    pub use super::super::http_bearer_token_ref::*;
}
mod http_content_type_text_ref {
    pub use super::super::http_content_type_text_ref::*;
}
mod http_cookie_headers_ref {
    pub use super::super::http_cookie_headers_ref::*;
}
mod http_cookie_name_ref {
    pub use super::super::http_cookie_name_ref::*;
}
mod http_cookie_value_ref {
    pub use super::super::http_cookie_value_ref::*;
}
mod optional_json_body_presence {
    pub use super::super::optional_json_body_presence::*;
}
mod optional_json_content_type {
    pub use super::super::optional_json_content_type::*;
}
mod optional_json_content_type_decision {
    pub use super::super::optional_json_content_type_decision::*;
}
mod resolve_bearer_authorization {
    pub use super::super::resolve_bearer_authorization::*;
}
mod resolve_optional_json_content_type_decision {
    pub use super::super::resolve_optional_json_content_type_decision::*;
}
mod resolve_unique_cookie {
    pub use super::super::resolve_unique_cookie::*;
}
