#[path = "classify_http_error_status.rs"]
mod classify_http_error_status;
#[path = "http_error_class.rs"]
mod http_error_class;
#[path = "http_error_status.rs"]
mod http_error_status;

pub use classify_http_error_status::classify_http_error_status;
pub use http_error_class::HttpErrorClass;
pub use http_error_status::HttpErrorStatus;

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
