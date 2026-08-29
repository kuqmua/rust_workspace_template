#[must_use]
pub const fn classify_http_error_status(
    status: crate::http_error_status::HttpErrorStatus,
) -> crate::http_error_class::HttpErrorClass {
    match status.0.as_u16() {
        200..=299 => crate::http_error_class::HttpErrorClass::UnexpectedSuccess,
        401 => crate::http_error_class::HttpErrorClass::Authentication,
        403 => crate::http_error_class::HttpErrorClass::Forbidden,
        404 => crate::http_error_class::HttpErrorClass::NotFound,
        408 | 504 => crate::http_error_class::HttpErrorClass::Timeout,
        409 => crate::http_error_class::HttpErrorClass::Conflict,
        413 => crate::http_error_class::HttpErrorClass::PayloadTooLarge,
        422 => crate::http_error_class::HttpErrorClass::Validation,
        429 => crate::http_error_class::HttpErrorClass::RateLimited,
        502 | 503 => crate::http_error_class::HttpErrorClass::ServiceUnavailable,
        _ => crate::http_error_class::HttpErrorClass::Internal,
    }
}
