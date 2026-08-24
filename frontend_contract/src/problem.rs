#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum ApiProblemKind {
    Authentication,
    Authorization,
    Conflict,
    InProgress,
    Internal,
    InvalidRequest,
    MethodNotAllowed,
    NotFound,
    PayloadTooLarge,
    Precondition,
    PreconditionRequired,
    RateLimited,
    RequestFailed,
    Validation,
}
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
    Internal(ApiProblemStatus),
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
    RequestFailed(ApiProblemStatus),
    #[error("API service is unavailable")]
    ServiceUnavailable,
    #[error("API request validation failed")]
    Validation,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    newtype::IntoInnerFrom,
    newtype::TryFrom,
)]
#[serde(try_from = "u16")]
#[try_from(
    error = crate::HttpStatusTryFromU16Error,
    validator = ApiProblemStatus::validate
)]
pub struct ApiProblemStatus(u16);
impl From<crate::KnownHttpStatus> for ApiProblemStatus {
    fn from(value: crate::KnownHttpStatus) -> Self {
        Self(value.get())
    }
}
impl ApiProblemStatus {
    #[allow(clippy::single_call_fn, clippy::trivially_copy_pass_by_ref)] // derive-generated TryFrom owns the single call and borrows the inner value
    fn validate(value: &u16) -> Result<(), crate::HttpStatusTryFromU16Error> {
        if (100u16..1_000u16).contains(value) {
            Ok(())
        } else {
            Err(crate::HttpStatusTryFromU16Error)
        }
    }
}
impl ApiProblemError {
    #[must_use]
    pub fn from_status(status: ApiProblemStatus) -> Self {
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
    pub fn status(self) -> ApiProblemStatus {
        match self {
            Self::Authentication => ApiProblemStatus::from(crate::KnownHttpStatus::Unauthorized),
            Self::Authorization => ApiProblemStatus::from(crate::KnownHttpStatus::Forbidden),
            Self::Conflict => ApiProblemStatus::from(crate::KnownHttpStatus::Conflict),
            Self::InProgress => ApiProblemStatus::from(crate::KnownHttpStatus::TooEarly),
            Self::Internal(status) | Self::RequestFailed(status) => status,
            Self::InvalidRequest => ApiProblemStatus::from(crate::KnownHttpStatus::BadRequest),
            Self::MethodNotAllowed => {
                ApiProblemStatus::from(crate::KnownHttpStatus::MethodNotAllowed)
            }
            Self::NotFound => ApiProblemStatus::from(crate::KnownHttpStatus::NotFound),
            Self::PayloadTooLarge => {
                ApiProblemStatus::from(crate::KnownHttpStatus::PayloadTooLarge)
            }
            Self::Precondition => {
                ApiProblemStatus::from(crate::KnownHttpStatus::PreconditionFailed)
            }
            Self::PreconditionRequired => {
                ApiProblemStatus::from(crate::KnownHttpStatus::PreconditionRequired)
            }
            Self::RateLimited => ApiProblemStatus::from(crate::KnownHttpStatus::TooManyRequests),
            Self::ServiceUnavailable => {
                ApiProblemStatus::from(crate::KnownHttpStatus::ServiceUnavailable)
            }
            Self::Validation => ApiProblemStatus::from(crate::KnownHttpStatus::UnprocessableEntity),
        }
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    newtype::AsRefStr,
    newtype::BoundedString,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
)]
#[bounded_string(max = 1024usize)]
#[serde(try_from = "String")]
pub struct ApiProblemDetail(String);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    newtype::BoundedString,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
)]
#[bounded_string(max = 128usize)]
#[serde(try_from = "String")]
pub struct ApiProblemRequestId(String);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    newtype::BoundedString,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
)]
#[bounded_string(max = 128usize)]
#[serde(try_from = "String")]
pub struct ApiProblemField(String);
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
pub struct ApiProblemViolation {
    detail: ApiProblemDetail,
    field: ApiProblemField,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    newtype::FromInner,
    serde::Deserialize,
    serde::Serialize,
)]
#[serde(
    from = "bounded_types::domain_types::vector::BoundedVec<ApiProblemViolation, { constants_usize::ZERO }, 128usize>"
)]
pub(crate) struct ApiProblemViolations(
    bounded_types::domain_types::vector::BoundedVec<
        ApiProblemViolation,
        { constants_usize::ZERO },
        128usize,
    >,
);
impl utoipa::PartialSchema for ApiProblemViolations {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        <bounded_types::domain_types::vector::BoundedVec<
            ApiProblemViolation,
            { constants_usize::ZERO },
            128usize,
        > as utoipa::PartialSchema>::schema()
    }
}
impl utoipa::ToSchema for ApiProblemViolations {}
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
    detail: ApiProblemDetail,
    request_id: Option<ApiProblemRequestId>,
    #[schema(inline)]
    violations: ApiProblemViolations,
    status: ApiProblemStatus,
    kind: ApiProblemKind,
}
impl ApiProblem {
    #[must_use]
    pub fn from_error(error: ApiProblemError) -> Self {
        let status = error.status();
        let (kind, detail) = match error {
            ApiProblemError::Authentication => (
                ApiProblemKind::Authentication,
                constants_str::AUTHENTICATION_REQUIRED,
            ),
            ApiProblemError::Authorization => (
                ApiProblemKind::Authorization,
                constants_str::AUTHORIZATION_FAILED,
            ),
            ApiProblemError::Conflict => (
                ApiProblemKind::Conflict,
                constants_str::RESOURCE_STATE_CONFLICT,
            ),
            ApiProblemError::InProgress => (
                ApiProblemKind::InProgress,
                constants_str::MATCHING_REQUEST_IS_STILL_IN_PROGRESS,
            ),
            ApiProblemError::Internal(_) | ApiProblemError::ServiceUnavailable => (
                ApiProblemKind::Internal,
                constants_str::INTERNAL_SERVER_ERROR,
            ),
            ApiProblemError::InvalidRequest => (
                ApiProblemKind::InvalidRequest,
                constants_str::INVALID_REQUEST,
            ),
            ApiProblemError::MethodNotAllowed => (
                ApiProblemKind::MethodNotAllowed,
                constants_str::METHOD_NOT_ALLOWED,
            ),
            ApiProblemError::NotFound => {
                (ApiProblemKind::NotFound, constants_str::RESOURCE_NOT_FOUND)
            }
            ApiProblemError::PayloadTooLarge => (
                ApiProblemKind::PayloadTooLarge,
                constants_str::REQUEST_BODY_IS_TOO_LARGE,
            ),
            ApiProblemError::Precondition => (
                ApiProblemKind::Precondition,
                constants_str::RESOURCE_PRECONDITION_FAILED,
            ),
            ApiProblemError::PreconditionRequired => (
                ApiProblemKind::PreconditionRequired,
                constants_str::REQUEST_PRECONDITION_IS_REQUIRED,
            ),
            ApiProblemError::RateLimited => (
                ApiProblemKind::RateLimited,
                constants_str::REQUEST_RATE_LIMIT_EXCEEDED_ALT,
            ),
            ApiProblemError::RequestFailed(_) => {
                (ApiProblemKind::RequestFailed, constants_str::REQUEST_FAILED)
            }
            ApiProblemError::Validation => (
                ApiProblemKind::Validation,
                constants_str::REQUEST_VALIDATION_FAILED,
            ),
        };
        Self {
            detail: ApiProblemDetail::try_from(detail.to_owned()).unwrap_or_default(),
            kind,
            request_id: None,
            status,
            violations: ApiProblemViolations::default(),
        }
    }
    #[must_use]
    pub const fn detail(&self) -> &ApiProblemDetail {
        &self.detail
    }
    #[must_use]
    pub const fn kind(&self) -> ApiProblemKind {
        self.kind
    }
    #[must_use]
    pub const fn status(&self) -> ApiProblemStatus {
        self.status
    }
}
#[cfg(not(target_arch = "wasm32"))]
impl axum::response::IntoResponse for ApiProblemError {
    fn into_response(self) -> axum::response::Response {
        let status = self.status();
        let mut response = axum::response::IntoResponse::into_response((
            axum::http::StatusCode::from_u16(u16::from(status))
                .unwrap_or(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
            axum::Json(ApiProblem::from_error(self)),
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

#[cfg(test)]
mod tests {
    #[test]
    #[allow(clippy::needless_for_each)] // workspace source policy requires iterator methods
    fn every_api_problem_error_is_an_error_enum_with_a_json_response() {
        fn assert_error<Error>()
        where
            Error: std::error::Error,
        {
        }

        assert_error::<super::ApiProblemError>();
        let internal_status = super::ApiProblemStatus::try_from(500u16)
            .expect("d2372bb7 assert_error invariant must hold");
        let request_failed_status = super::ApiProblemStatus::try_from(418u16)
            .expect("805da7f4 assert_error invariant must hold");
        [
            (
                super::ApiProblemError::InvalidRequest,
                400u16,
                super::ApiProblemKind::InvalidRequest,
            ),
            (
                super::ApiProblemError::Authentication,
                401u16,
                super::ApiProblemKind::Authentication,
            ),
            (
                super::ApiProblemError::Authorization,
                403u16,
                super::ApiProblemKind::Authorization,
            ),
            (
                super::ApiProblemError::NotFound,
                404u16,
                super::ApiProblemKind::NotFound,
            ),
            (
                super::ApiProblemError::MethodNotAllowed,
                405u16,
                super::ApiProblemKind::MethodNotAllowed,
            ),
            (
                super::ApiProblemError::Conflict,
                409u16,
                super::ApiProblemKind::Conflict,
            ),
            (
                super::ApiProblemError::Precondition,
                412u16,
                super::ApiProblemKind::Precondition,
            ),
            (
                super::ApiProblemError::PayloadTooLarge,
                413u16,
                super::ApiProblemKind::PayloadTooLarge,
            ),
            (
                super::ApiProblemError::Validation,
                422u16,
                super::ApiProblemKind::Validation,
            ),
            (
                super::ApiProblemError::InProgress,
                425u16,
                super::ApiProblemKind::InProgress,
            ),
            (
                super::ApiProblemError::PreconditionRequired,
                428u16,
                super::ApiProblemKind::PreconditionRequired,
            ),
            (
                super::ApiProblemError::RateLimited,
                429u16,
                super::ApiProblemKind::RateLimited,
            ),
            (
                super::ApiProblemError::Internal(internal_status),
                500u16,
                super::ApiProblemKind::Internal,
            ),
            (
                super::ApiProblemError::ServiceUnavailable,
                503u16,
                super::ApiProblemKind::Internal,
            ),
            (
                super::ApiProblemError::RequestFailed(request_failed_status),
                418u16,
                super::ApiProblemKind::RequestFailed,
            ),
        ]
        .into_iter()
        .for_each(|(error, status, kind)| {
            let response = axum::response::IntoResponse::into_response(error);
            assert_eq!(response.status().as_u16(), status);
            assert_eq!(
                response.headers().get(axum::http::header::CONTENT_TYPE),
                Some(&axum::http::HeaderValue::from_static(
                    constants_str::APPLICATION_PROBLEM_PLUS_JSON
                ))
            );
            assert_eq!(
                response
                    .headers()
                    .contains_key(axum::http::header::RETRY_AFTER),
                status == 429u16
            );
            let body = futures::executor::block_on(axum::body::to_bytes(
                response.into_body(),
                16_384usize,
            ))
            .expect("3e43e7bc assert_error invariant must hold");
            let problem = serde_json::from_slice::<super::ApiProblem>(&body)
                .expect("116dc695 assert_error invariant must hold");
            assert_eq!(u16::from(problem.status()), status);
            assert_eq!(problem.kind(), kind);
        });
    }

    #[test]
    fn problem_text_deserialization_uses_bounded_try_from() {
        let detail = serde_json::to_string(&"x".repeat(1_025usize)).expect(
            "6e2db8a1 problem_text_deserialization_uses_bounded_try_from invariant must hold",
        );
        let request_id = serde_json::to_string(&"x".repeat(129usize)).expect(
            "f289a40c problem_text_deserialization_uses_bounded_try_from invariant must hold",
        );
        let _detail_error =
            serde_json::from_str::<super::ApiProblemDetail>(&detail).expect_err("b653c1c0");
        let _field_error =
            serde_json::from_str::<super::ApiProblemField>(&request_id).expect_err("0b04a860");
        let _request_id_error =
            serde_json::from_str::<super::ApiProblemRequestId>(&request_id).expect_err("abe1c2e7");
    }

    #[test]
    fn problem_violation_deserialization_rejects_too_many_items() {
        let item = serde_json::json!({
            "detail": "invalid",
            "field": "name"
        });
        let serialized = serde_json::to_string(&vec![item; 129usize]).expect(
            "a1010d3f problem_violation_deserialization_rejects_too_many_items invariant must hold",
        );
        let _error =
            serde_json::from_str::<super::ApiProblemViolations>(&serialized).expect_err("a05e84a8");
    }
}
