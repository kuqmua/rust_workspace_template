#[path = "bearer_authorization_resolution.rs"]
mod bearer_authorization_resolution;
#[path = "classify_optional_json_content_type.rs"]
mod classify_optional_json_content_type;
#[path = "cookie_resolution.rs"]
mod cookie_resolution;
#[path = "http_authorization_header_text_ref.rs"]
mod http_authorization_header_text_ref;
#[path = "http_bearer_token_ref.rs"]
mod http_bearer_token_ref;
#[path = "http_content_type_text_ref.rs"]
mod http_content_type_text_ref;
#[path = "http_cookie_headers_ref.rs"]
mod http_cookie_headers_ref;
#[path = "http_cookie_name_ref.rs"]
mod http_cookie_name_ref;
#[path = "http_cookie_value_ref.rs"]
mod http_cookie_value_ref;
#[path = "optional_json_body_presence.rs"]
mod optional_json_body_presence;
#[path = "optional_json_content_type.rs"]
mod optional_json_content_type;
#[path = "optional_json_content_type_decision.rs"]
mod optional_json_content_type_decision;
#[path = "resolve_bearer_authorization.rs"]
mod resolve_bearer_authorization;
#[path = "resolve_optional_json_content_type_decision.rs"]
mod resolve_optional_json_content_type_decision;
#[path = "resolve_unique_cookie.rs"]
mod resolve_unique_cookie;

pub use bearer_authorization_resolution::BearerAuthorizationResolution;
pub use classify_optional_json_content_type::classify_optional_json_content_type;
pub use cookie_resolution::CookieResolution;
pub use http_authorization_header_text_ref::HttpAuthorizationHeaderTextRef;
pub use http_bearer_token_ref::HttpBearerTokenRef;
pub use http_content_type_text_ref::HttpContentTypeTextRef;
pub use http_cookie_headers_ref::HttpCookieHeadersRef;
pub use http_cookie_name_ref::HttpCookieNameRef;
pub use http_cookie_value_ref::HttpCookieValueRef;
pub use optional_json_body_presence::OptionalJsonBodyPresence;
pub use optional_json_content_type::OptionalJsonContentType;
pub use optional_json_content_type_decision::OptionalJsonContentTypeDecision;
pub use resolve_bearer_authorization::resolve_bearer_authorization;
pub use resolve_optional_json_content_type_decision::resolve_optional_json_content_type_decision;
pub use resolve_unique_cookie::resolve_unique_cookie;

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
