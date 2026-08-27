#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    PartialEq,
    Eq,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
)]
pub struct ApiProblem {
    detail: super::ApiProblemDetail,
    request_id: Option<super::ApiProblemRequestId>,
    #[schema(inline)]
    violations: super::ApiProblemViolations,
    status: super::ApiProblemStatus,
    kind: super::ApiProblemKind,
}

impl ApiProblem {
    #[must_use]
    pub fn from_error(error: super::ApiProblemError) -> Self {
        let status = error.status();
        let (kind, detail) = match error {
            super::ApiProblemError::Authentication => (
                super::ApiProblemKind::Authentication,
                constants_str::AUTHENTICATION_REQUIRED,
            ),
            super::ApiProblemError::Authorization => (
                super::ApiProblemKind::Authorization,
                constants_str::AUTHORIZATION_FAILED,
            ),
            super::ApiProblemError::Conflict => (
                super::ApiProblemKind::Conflict,
                constants_str::RESOURCE_STATE_CONFLICT,
            ),
            super::ApiProblemError::InProgress => (
                super::ApiProblemKind::InProgress,
                constants_str::MATCHING_REQUEST_IS_STILL_IN_PROGRESS,
            ),
            super::ApiProblemError::Internal(_) | super::ApiProblemError::ServiceUnavailable => (
                super::ApiProblemKind::Internal,
                constants_str::INTERNAL_SERVER_ERROR,
            ),
            super::ApiProblemError::InvalidRequest => (
                super::ApiProblemKind::InvalidRequest,
                constants_str::INVALID_REQUEST,
            ),
            super::ApiProblemError::MethodNotAllowed => (
                super::ApiProblemKind::MethodNotAllowed,
                constants_str::METHOD_NOT_ALLOWED,
            ),
            super::ApiProblemError::NotFound => (
                super::ApiProblemKind::NotFound,
                constants_str::RESOURCE_NOT_FOUND,
            ),
            super::ApiProblemError::PayloadTooLarge => (
                super::ApiProblemKind::PayloadTooLarge,
                constants_str::REQUEST_BODY_IS_TOO_LARGE,
            ),
            super::ApiProblemError::Precondition => (
                super::ApiProblemKind::Precondition,
                constants_str::RESOURCE_PRECONDITION_FAILED,
            ),
            super::ApiProblemError::PreconditionRequired => (
                super::ApiProblemKind::PreconditionRequired,
                constants_str::REQUEST_PRECONDITION_IS_REQUIRED,
            ),
            super::ApiProblemError::RateLimited => (
                super::ApiProblemKind::RateLimited,
                constants_str::REQUEST_RATE_LIMIT_EXCEEDED_ALT,
            ),
            super::ApiProblemError::RequestFailed(_) => (
                super::ApiProblemKind::RequestFailed,
                constants_str::REQUEST_FAILED,
            ),
            super::ApiProblemError::Validation => (
                super::ApiProblemKind::Validation,
                constants_str::REQUEST_VALIDATION_FAILED,
            ),
        };
        Self {
            detail: super::ApiProblemDetail::try_from(detail.to_owned()).unwrap_or_default(),
            kind,
            request_id: None,
            status,
            violations: super::ApiProblemViolations::default(),
        }
    }

    #[must_use]
    pub const fn detail(&self) -> &super::ApiProblemDetail {
        &self.detail
    }
    #[must_use]
    pub const fn kind(&self) -> super::ApiProblemKind {
        self.kind
    }
    #[must_use]
    pub const fn status(&self) -> super::ApiProblemStatus {
        self.status
    }
}
