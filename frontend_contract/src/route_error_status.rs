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
    pub fn transport_status(self) -> crate::TransportStatus {
        let status = match self {
            Self::Authentication => crate::KnownHttpStatus::Unauthorized,
            Self::Authorization => crate::KnownHttpStatus::Forbidden,
            Self::Conflict => crate::KnownHttpStatus::Conflict,
            Self::Internal => crate::KnownHttpStatus::InternalServerError,
            Self::MethodNotAllowed => crate::KnownHttpStatus::MethodNotAllowed,
            Self::PayloadTooLarge => crate::KnownHttpStatus::PayloadTooLarge,
            Self::RateLimited => crate::KnownHttpStatus::TooManyRequests,
            Self::ServiceUnavailable => crate::KnownHttpStatus::ServiceUnavailable,
            Self::Validation => crate::KnownHttpStatus::UnprocessableEntity,
        };
        crate::TransportStatus::from(status)
    }
}
