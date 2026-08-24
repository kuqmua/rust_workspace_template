#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
)]
pub struct HttpErrorStatus(http::StatusCode);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
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
    #[allow(clippy::needless_for_each)] // iterator form is required by the workspace no-for-loop policy
    fn representative_statuses_cover_every_error_class() {
        [
            (
                http::StatusCode::OK,
                super::HttpErrorClass::UnexpectedSuccess,
            ),
            (
                http::StatusCode::UNAUTHORIZED,
                super::HttpErrorClass::Authentication,
            ),
            (
                http::StatusCode::FORBIDDEN,
                super::HttpErrorClass::Forbidden,
            ),
            (http::StatusCode::NOT_FOUND, super::HttpErrorClass::NotFound),
            (
                http::StatusCode::REQUEST_TIMEOUT,
                super::HttpErrorClass::Timeout,
            ),
            (
                http::StatusCode::GATEWAY_TIMEOUT,
                super::HttpErrorClass::Timeout,
            ),
            (http::StatusCode::CONFLICT, super::HttpErrorClass::Conflict),
            (
                http::StatusCode::PAYLOAD_TOO_LARGE,
                super::HttpErrorClass::PayloadTooLarge,
            ),
            (
                http::StatusCode::UNPROCESSABLE_ENTITY,
                super::HttpErrorClass::Validation,
            ),
            (
                http::StatusCode::TOO_MANY_REQUESTS,
                super::HttpErrorClass::RateLimited,
            ),
            (
                http::StatusCode::BAD_GATEWAY,
                super::HttpErrorClass::ServiceUnavailable,
            ),
            (
                http::StatusCode::SERVICE_UNAVAILABLE,
                super::HttpErrorClass::ServiceUnavailable,
            ),
            (
                http::StatusCode::BAD_REQUEST,
                super::HttpErrorClass::Internal,
            ),
        ]
        .into_iter()
        .for_each(|(status, expected)| {
            assert_eq!(super::classify_http_error_status(status.into()), expected);
        });
    }
}
