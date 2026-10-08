#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq,
)]
pub enum KnownHttpStatus {
    BadRequest,
    Conflict,
    Created,
    Forbidden,
    InternalServerError,
    MethodNotAllowed,
    NoContent,
    NotFound,
    Ok,
    PayloadTooLarge,
    PreconditionFailed,
    PreconditionRequired,
    ServiceUnavailable,
    TooEarly,
    TooManyRequests,
    Unauthorized,
    UnprocessableEntity,
}

impl KnownHttpStatus {
    #[must_use]
    pub const fn get(self) -> u16 {
        match self {
            Self::BadRequest => 400u16,
            Self::Conflict => 409u16,
            Self::Created => 201u16,
            Self::Forbidden => 403u16,
            Self::InternalServerError => 500u16,
            Self::MethodNotAllowed => 405u16,
            Self::NoContent => 204u16,
            Self::NotFound => 404u16,
            Self::Ok => 200u16,
            Self::PayloadTooLarge => 413u16,
            Self::PreconditionFailed => 412u16,
            Self::PreconditionRequired => 428u16,
            Self::ServiceUnavailable => 503u16,
            Self::TooEarly => 425u16,
            Self::TooManyRequests => 429u16,
            Self::Unauthorized => 401u16,
            Self::UnprocessableEntity => 422u16,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_known_status_preserves_protocol_code() {
        let cases = [
            (
                crate::known_http_status::KnownHttpStatus::BadRequest,
                400u16,
            ),
            (crate::known_http_status::KnownHttpStatus::Conflict, 409u16),
            (crate::known_http_status::KnownHttpStatus::Created, 201u16),
            (crate::known_http_status::KnownHttpStatus::Forbidden, 403u16),
            (
                crate::known_http_status::KnownHttpStatus::InternalServerError,
                500u16,
            ),
            (
                crate::known_http_status::KnownHttpStatus::MethodNotAllowed,
                405u16,
            ),
            (crate::known_http_status::KnownHttpStatus::NoContent, 204u16),
            (crate::known_http_status::KnownHttpStatus::NotFound, 404u16),
            (crate::known_http_status::KnownHttpStatus::Ok, 200u16),
            (
                crate::known_http_status::KnownHttpStatus::PayloadTooLarge,
                413u16,
            ),
            (
                crate::known_http_status::KnownHttpStatus::PreconditionFailed,
                412u16,
            ),
            (
                crate::known_http_status::KnownHttpStatus::PreconditionRequired,
                428u16,
            ),
            (
                crate::known_http_status::KnownHttpStatus::ServiceUnavailable,
                503u16,
            ),
            (crate::known_http_status::KnownHttpStatus::TooEarly, 425u16),
            (
                crate::known_http_status::KnownHttpStatus::TooManyRequests,
                429u16,
            ),
            (
                crate::known_http_status::KnownHttpStatus::Unauthorized,
                401u16,
            ),
            (
                crate::known_http_status::KnownHttpStatus::UnprocessableEntity,
                422u16,
            ),
        ];
        assert!(cases.into_iter().all(|(known_http_status, expected_code)| {
            known_http_status.get() == expected_code
                && u16::from(crate::transport_status::TransportStatus::from(
                    known_http_status,
                )) == expected_code
        }));
    }
}
