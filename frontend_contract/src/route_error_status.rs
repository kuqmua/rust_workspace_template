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
    pub fn transport_status(self) -> super::super::TransportStatus {
        let status = match self {
            Self::Authentication => super::super::KnownHttpStatus::Unauthorized,
            Self::Authorization => super::super::KnownHttpStatus::Forbidden,
            Self::Conflict => super::super::KnownHttpStatus::Conflict,
            Self::Internal => super::super::KnownHttpStatus::InternalServerError,
            Self::MethodNotAllowed => super::super::KnownHttpStatus::MethodNotAllowed,
            Self::PayloadTooLarge => super::super::KnownHttpStatus::PayloadTooLarge,
            Self::RateLimited => super::super::KnownHttpStatus::TooManyRequests,
            Self::ServiceUnavailable => super::super::KnownHttpStatus::ServiceUnavailable,
            Self::Validation => super::super::KnownHttpStatus::UnprocessableEntity,
        };
        super::super::TransportStatus::from(status)
    }
}
