use super::allow_origin_suffix::AllowOriginSuffix;
pub use super::allowed_origin::AllowedOrigin;
pub use super::allowed_origin_error::AllowedOriginError;
pub use super::allowed_origins::AllowedOrigins;
pub use super::allowed_origins_error::AllowedOriginsError;
use super::http_origin_authority_text::HttpOriginAuthorityText;
pub use super::http_origin_headers_ref::HttpOriginHeadersRef;
use super::http_origin_scheme_text::HttpOriginSchemeText;
use super::http_origin_text_ref::HttpOriginTextRef;
use super::parsed_http_origin_ref::ParsedHttpOriginRef;
pub use super::request_origin_allowed::RequestOriginAllowed;
use super::request_origin_value_is_allowed::request_origin_value_is_allowed;
pub use super::resolve_request_origin_allowed::resolve_request_origin_allowed;
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

// Root-owned module compatibility wrappers.
mod allow_origin_suffix {
    pub use super::super::allow_origin_suffix::*;
}
mod allowed_origin {
    pub use super::super::allowed_origin::*;
}
mod allowed_origin_error {
    pub use super::super::allowed_origin_error::*;
}
mod allowed_origins {
    pub use super::super::allowed_origins::*;
}
mod allowed_origins_error {
    pub use super::super::allowed_origins_error::*;
}
mod http_origin_authority_text {
    pub use super::super::http_origin_authority_text::*;
}
mod http_origin_headers_ref {
    pub use super::super::http_origin_headers_ref::*;
}
mod http_origin_scheme_text {
    pub use super::super::http_origin_scheme_text::*;
}
mod http_origin_text_ref {
    pub use super::super::http_origin_text_ref::*;
}
mod parsed_http_origin_ref {
    pub use super::super::parsed_http_origin_ref::*;
}
mod request_origin_allowed {
    pub use super::super::request_origin_allowed::*;
}
mod request_origin_value_is_allowed {
    pub use super::super::request_origin_value_is_allowed::*;
}
mod resolve_request_origin_allowed {
    pub use super::super::resolve_request_origin_allowed::*;
}
