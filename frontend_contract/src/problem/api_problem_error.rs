#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum ApiProblemError {
    #[error("API authentication failed")]
    Authentication,
    #[error("API authorization failed")]
    Authorization,
    #[error("API operation conflicts with current state")]
    Conflict,
    #[error("API request is still in progress")]
    InProgress,
    #[error("internal API operation failed")]
    Internal(super::ApiProblemStatus),
    #[error("API request is invalid")]
    InvalidRequest,
    #[error("API route does not support this HTTP method")]
    MethodNotAllowed,
    #[error("API resource was not found")]
    NotFound,
    #[error("API request body is too large")]
    PayloadTooLarge,
    #[error("API resource precondition failed")]
    Precondition,
    #[error("API request precondition is required")]
    PreconditionRequired,
    #[error("API request rate limit was exceeded")]
    RateLimited,
    #[error("API request failed")]
    RequestFailed(super::ApiProblemStatus),
    #[error("API service is unavailable")]
    ServiceUnavailable,
    #[error("API request validation failed")]
    Validation,
}

impl ApiProblemError {
    #[must_use]
    pub fn from_status(status: super::ApiProblemStatus) -> Self {
        match u16::from(status) {
            400u16 => Self::InvalidRequest,
            401u16 => Self::Authentication,
            403u16 => Self::Authorization,
            404u16 => Self::NotFound,
            405u16 => Self::MethodNotAllowed,
            409u16 => Self::Conflict,
            412u16 => Self::Precondition,
            413u16 => Self::PayloadTooLarge,
            422u16 => Self::Validation,
            425u16 => Self::InProgress,
            428u16 => Self::PreconditionRequired,
            429u16 => Self::RateLimited,
            503u16 => Self::ServiceUnavailable,
            500u16..=599u16 => Self::Internal(status),
            _ => Self::RequestFailed(status),
        }
    }

    #[must_use]
    pub fn status(self) -> super::ApiProblemStatus {
        let status = match self {
            Self::Authentication => crate::domain_types::KnownHttpStatus::Unauthorized,
            Self::Authorization => crate::domain_types::KnownHttpStatus::Forbidden,
            Self::Conflict => crate::domain_types::KnownHttpStatus::Conflict,
            Self::InProgress => crate::domain_types::KnownHttpStatus::TooEarly,
            Self::Internal(status) | Self::RequestFailed(status) => return status,
            Self::InvalidRequest => crate::domain_types::KnownHttpStatus::BadRequest,
            Self::MethodNotAllowed => crate::domain_types::KnownHttpStatus::MethodNotAllowed,
            Self::NotFound => crate::domain_types::KnownHttpStatus::NotFound,
            Self::PayloadTooLarge => crate::domain_types::KnownHttpStatus::PayloadTooLarge,
            Self::Precondition => crate::domain_types::KnownHttpStatus::PreconditionFailed,
            Self::PreconditionRequired => {
                crate::domain_types::KnownHttpStatus::PreconditionRequired
            }
            Self::RateLimited => crate::domain_types::KnownHttpStatus::TooManyRequests,
            Self::ServiceUnavailable => crate::domain_types::KnownHttpStatus::ServiceUnavailable,
            Self::Validation => crate::domain_types::KnownHttpStatus::UnprocessableEntity,
        };
        super::ApiProblemStatus::from(status)
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl axum::response::IntoResponse for ApiProblemError {
    fn into_response(self) -> axum::response::Response {
        let status = self.status();
        let mut response = axum::response::IntoResponse::into_response((
            axum::http::StatusCode::from_u16(u16::from(status))
                .unwrap_or(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
            axum::Json(super::ApiProblem::from_error(self)),
        ));
        let _previous_content_type = response.headers_mut().insert(
            axum::http::header::CONTENT_TYPE,
            axum::http::HeaderValue::from_static(constants_str::APPLICATION_PROBLEM_PLUS_JSON),
        );
        if self == Self::RateLimited {
            let _previous_retry_after = response.headers_mut().insert(
                axum::http::header::RETRY_AFTER,
                axum::http::HeaderValue::from_static(constants_str::VALUE_60),
            );
        }
        response
    }
}
