#[must_use]
pub const fn classify_http_error_status(status: super::HttpErrorStatus) -> super::HttpErrorClass {
    match status.0.as_u16() {
        200..=299 => super::HttpErrorClass::UnexpectedSuccess,
        401 => super::HttpErrorClass::Authentication,
        403 => super::HttpErrorClass::Forbidden,
        404 => super::HttpErrorClass::NotFound,
        408 | 504 => super::HttpErrorClass::Timeout,
        409 => super::HttpErrorClass::Conflict,
        413 => super::HttpErrorClass::PayloadTooLarge,
        422 => super::HttpErrorClass::Validation,
        429 => super::HttpErrorClass::RateLimited,
        502 | 503 => super::HttpErrorClass::ServiceUnavailable,
        _ => super::HttpErrorClass::Internal,
    }
}
