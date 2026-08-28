#[path = "allow_origin_suffix.rs"]
mod allow_origin_suffix;
#[path = "allowed_origin.rs"]
mod allowed_origin;
#[path = "allowed_origin_error.rs"]
mod allowed_origin_error;
#[path = "allowed_origins.rs"]
mod allowed_origins;
#[path = "allowed_origins_error.rs"]
mod allowed_origins_error;
#[path = "http_origin_authority_text.rs"]
mod http_origin_authority_text;
#[path = "http_origin_headers_ref.rs"]
mod http_origin_headers_ref;
#[path = "http_origin_scheme_text.rs"]
mod http_origin_scheme_text;
#[path = "http_origin_text_ref.rs"]
mod http_origin_text_ref;
#[path = "parsed_http_origin_ref.rs"]
mod parsed_http_origin_ref;
#[path = "request_origin_allowed.rs"]
mod request_origin_allowed;
#[path = "request_origin_value_is_allowed.rs"]
mod request_origin_value_is_allowed;
#[path = "resolve_request_origin_allowed.rs"]
mod resolve_request_origin_allowed;

use allow_origin_suffix::AllowOriginSuffix;
pub use allowed_origin::AllowedOrigin;
pub use allowed_origin_error::AllowedOriginError;
pub use allowed_origins::AllowedOrigins;
pub use allowed_origins_error::AllowedOriginsError;
use http_origin_authority_text::HttpOriginAuthorityText;
pub use http_origin_headers_ref::HttpOriginHeadersRef;
use http_origin_scheme_text::HttpOriginSchemeText;
use http_origin_text_ref::HttpOriginTextRef;
use parsed_http_origin_ref::ParsedHttpOriginRef;
pub use request_origin_allowed::RequestOriginAllowed;
use request_origin_value_is_allowed::request_origin_value_is_allowed;
pub use resolve_request_origin_allowed::resolve_request_origin_allowed;

#[cfg(test)]
mod tests {
    fn allowed_origins() -> super::AllowedOrigins {
        super::AllowedOrigins::try_from(vec![String::from(constants_str::HTTPS_ADMIN_EXAMPLE_COM)])
            .expect("782d2bed allowed_origins invariant must hold")
    }

    #[test]
    fn allowed_origins_reject_oversized_lists() {
        let values = vec![String::from(constants_str::HTTPS_ADMIN_EXAMPLE_COM); 129usize];
        assert_eq!(
            super::AllowedOrigins::try_from(values),
            Err(super::AllowedOriginsError)
        );
    }

    #[test]
    fn allowed_origins_reject_userinfo_and_invalid_ports() {
        assert_eq!(
            super::AllowedOrigin::try_from(String::from(
                constants_str::HTTPS_ADMIN_EXAMPLE_COM_WITH_USERINFO,
            )),
            Err(super::AllowedOriginError)
        );
        assert_eq!(
            super::AllowedOrigin::try_from(String::from(
                constants_str::HTTPS_ADMIN_EXAMPLE_COM_WITH_INVALID_PORT,
            )),
            Err(super::AllowedOriginError)
        );
    }

    #[test]
    fn origin_requires_exact_authority_without_path() {
        let mut headers = http::HeaderMap::new();
        let _previous = headers.insert(
            http::header::ORIGIN,
            http::HeaderValue::from_static(constants_str::HTTPS_ADMIN_EXAMPLE_COM_PATH),
        );
        assert!(!bool::from(super::resolve_request_origin_allowed(
            super::HttpOriginHeadersRef::from(&headers),
            &allowed_origins(),
        )));
    }

    #[test]
    fn referer_accepts_path_and_compares_case_insensitively() {
        let mut headers = http::HeaderMap::new();
        let _previous = headers.insert(
            http::header::REFERER,
            http::HeaderValue::from_static(constants_str::HTTPS_ADMIN_EXAMPLE_COM_SETTINGS_UPPER),
        );
        assert!(bool::from(super::resolve_request_origin_allowed(
            super::HttpOriginHeadersRef::from(&headers),
            &allowed_origins(),
        )));
    }
}
