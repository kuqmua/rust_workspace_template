#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
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
        assert_eq!(super::KnownHttpStatus::TooManyRequests.get(), 429u16);
    }
}
