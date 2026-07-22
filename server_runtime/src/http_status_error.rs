#[derive(Clone, Copy, Debug, Eq, PartialEq, newtype::FromInner)]
pub struct HttpErrorStatus(http::StatusCode);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HttpErrorClass {
    Authentication,
    Conflict,
    Forbidden,
    Internal,
    NotFound,
    PayloadTooLarge,
    RateLimited,
    ServiceUnavailable,
    Timeout,
    UnexpectedSuccess,
    Validation,
}

#[must_use]
pub const fn classify_http_error_status(status: HttpErrorStatus) -> HttpErrorClass {
    match status.0.as_u16() {
        200..=299 => HttpErrorClass::UnexpectedSuccess,
        401 => HttpErrorClass::Authentication,
        403 => HttpErrorClass::Forbidden,
        404 => HttpErrorClass::NotFound,
        408 | 504 => HttpErrorClass::Timeout,
        409 => HttpErrorClass::Conflict,
        413 => HttpErrorClass::PayloadTooLarge,
        422 => HttpErrorClass::Validation,
        429 => HttpErrorClass::RateLimited,
        502 | 503 => HttpErrorClass::ServiceUnavailable,
        _ => HttpErrorClass::Internal,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn known_statuses_have_stable_error_classes() {
        assert_eq!(
            super::classify_http_error_status(http::StatusCode::CONFLICT.into()),
            super::HttpErrorClass::Conflict,
        );
        assert_eq!(
            super::classify_http_error_status(http::StatusCode::PAYLOAD_TOO_LARGE.into()),
            super::HttpErrorClass::PayloadTooLarge,
        );
        assert_eq!(
            super::classify_http_error_status(http::StatusCode::SERVICE_UNAVAILABLE.into()),
            super::HttpErrorClass::ServiceUnavailable,
        );
    }
}
