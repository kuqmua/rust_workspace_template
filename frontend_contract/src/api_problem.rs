#[derive(generate_accessor::Getters)]
#[getters(bare)]
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
    detail: crate::api_problem_detail::ApiProblemDetail,
    request_id: Option<crate::api_problem_request_id::ApiProblemRequestId>,
    #[schema(inline)]
    violations: crate::api_problem_violations::ApiProblemViolations,
    #[getters(copy)]
    status: crate::api_problem_status::ApiProblemStatus,
    #[getters(copy)]
    kind: crate::api_problem_kind::ApiProblemKind,
}

impl ApiProblem {
    #[must_use]
    pub fn from_error(error: crate::api_problem_error::ApiProblemError) -> Self {
        let status = error.status();
        let (kind, detail) = match error {
            crate::api_problem_error::ApiProblemError::Authentication => (
                crate::api_problem_kind::ApiProblemKind::Authentication,
                constants_str::AUTHENTICATION_REQUIRED,
            ),
            crate::api_problem_error::ApiProblemError::Authorization => (
                crate::api_problem_kind::ApiProblemKind::Authorization,
                constants_str::AUTHORIZATION_FAILED,
            ),
            crate::api_problem_error::ApiProblemError::Conflict => (
                crate::api_problem_kind::ApiProblemKind::Conflict,
                constants_str::RESOURCE_STATE_CONFLICT,
            ),
            crate::api_problem_error::ApiProblemError::InProgress => (
                crate::api_problem_kind::ApiProblemKind::InProgress,
                constants_str::MATCHING_REQUEST_IS_STILL_IN_PROGRESS,
            ),
            crate::api_problem_error::ApiProblemError::Internal(_)
            | crate::api_problem_error::ApiProblemError::ServiceUnavailable => (
                crate::api_problem_kind::ApiProblemKind::Internal,
                constants_str::INTERNAL_SERVER_ERROR,
            ),
            crate::api_problem_error::ApiProblemError::InvalidRequest => (
                crate::api_problem_kind::ApiProblemKind::InvalidRequest,
                constants_str::INVALID_REQUEST,
            ),
            crate::api_problem_error::ApiProblemError::MethodNotAllowed => (
                crate::api_problem_kind::ApiProblemKind::MethodNotAllowed,
                constants_str::METHOD_NOT_ALLOWED,
            ),
            crate::api_problem_error::ApiProblemError::NotFound => (
                crate::api_problem_kind::ApiProblemKind::NotFound,
                constants_str::RESOURCE_NOT_FOUND,
            ),
            crate::api_problem_error::ApiProblemError::PayloadTooLarge => (
                crate::api_problem_kind::ApiProblemKind::PayloadTooLarge,
                constants_str::REQUEST_BODY_IS_TOO_LARGE,
            ),
            crate::api_problem_error::ApiProblemError::Precondition => (
                crate::api_problem_kind::ApiProblemKind::Precondition,
                constants_str::RESOURCE_PRECONDITION_FAILED,
            ),
            crate::api_problem_error::ApiProblemError::PreconditionRequired => (
                crate::api_problem_kind::ApiProblemKind::PreconditionRequired,
                constants_str::REQUEST_PRECONDITION_IS_REQUIRED,
            ),
            crate::api_problem_error::ApiProblemError::RateLimited => (
                crate::api_problem_kind::ApiProblemKind::RateLimited,
                constants_str::REQUEST_RATE_LIMIT_EXCEEDED_ALT,
            ),
            crate::api_problem_error::ApiProblemError::RequestFailed(_) => (
                crate::api_problem_kind::ApiProblemKind::RequestFailed,
                constants_str::REQUEST_FAILED,
            ),
            crate::api_problem_error::ApiProblemError::Validation => (
                crate::api_problem_kind::ApiProblemKind::Validation,
                constants_str::REQUEST_VALIDATION_FAILED,
            ),
        };
        Self {
            detail: crate::api_problem_detail::ApiProblemDetail::try_from(detail.to_owned())
                .unwrap_or_default(),
            kind,
            request_id: None,
            status,
            violations: crate::api_problem_violations::ApiProblemViolations::default(),
        }
    }
}
