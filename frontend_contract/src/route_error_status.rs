#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq)]
pub enum RouteErrorStatus {
    Authentication,
    Authorization,
    Conflict,
    Internal,
    MethodNotAllowed,
    PayloadTooLarge,
    RateLimited,
    ServiceUnavailable,
    Validation,
}

impl RouteErrorStatus {
    #[must_use]
    pub fn transport_status(self) -> crate::transport_status::TransportStatus {
        let status = match self {
            Self::Authentication => crate::known_http_status::KnownHttpStatus::Unauthorized,
            Self::Authorization => crate::known_http_status::KnownHttpStatus::Forbidden,
            Self::Conflict => crate::known_http_status::KnownHttpStatus::Conflict,
            Self::Internal => crate::known_http_status::KnownHttpStatus::InternalServerError,
            Self::MethodNotAllowed => crate::known_http_status::KnownHttpStatus::MethodNotAllowed,
            Self::PayloadTooLarge => crate::known_http_status::KnownHttpStatus::PayloadTooLarge,
            Self::RateLimited => crate::known_http_status::KnownHttpStatus::TooManyRequests,
            Self::ServiceUnavailable => {
                crate::known_http_status::KnownHttpStatus::ServiceUnavailable
            }
            Self::Validation => crate::known_http_status::KnownHttpStatus::UnprocessableEntity,
        };
        crate::transport_status::TransportStatus::from(status)
    }
}
