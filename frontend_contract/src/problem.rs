#[derive(
    Clone, Copy, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize, utoipa::ToSchema,
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
    Clone, Copy, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize, utoipa::ToSchema,
)]
#[serde(transparent)]
pub struct ApiProblemStatus(u16);
impl From<u16> for ApiProblemStatus {
    fn from(value: u16) -> Self {
        Self(value)
    }
}
impl From<ApiProblemStatus> for u16 {
    fn from(value: ApiProblemStatus) -> Self {
        value.0
    }
}
#[derive(
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
#[bounded_string(max = 1024usize)]
#[serde(transparent)]
pub struct ApiProblemDetail(String);
impl AsRef<str> for ApiProblemDetail {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
#[derive(
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
#[serde(transparent)]
pub struct ApiProblemRequestId(String);
#[derive(
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
#[serde(transparent)]
pub struct ApiProblemField(String);
#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
pub struct ApiProblemViolation {
    detail: ApiProblemDetail,
    field: ApiProblemField,
}
#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
pub struct ApiProblem {
    detail: ApiProblemDetail,
    kind: ApiProblemKind,
    request_id: Option<ApiProblemRequestId>,
    status: ApiProblemStatus,
    violations: Vec<ApiProblemViolation>,
}
impl ApiProblem {
    #[must_use]
    pub fn from_status(status: ApiProblemStatus) -> Self {
        let (kind, detail) = match u16::from(status) {
            400u16 => (ApiProblemKind::InvalidRequest, "invalid request"),
            401u16 => (ApiProblemKind::Authentication, "authentication required"),
            403u16 => (ApiProblemKind::Authorization, "authorization failed"),
            404u16 => (ApiProblemKind::NotFound, "resource not found"),
            405u16 => (ApiProblemKind::MethodNotAllowed, "method not allowed"),
            409u16 => (ApiProblemKind::Conflict, "resource state conflict"),
            412u16 => (ApiProblemKind::Precondition, "resource precondition failed"),
            413u16 => (ApiProblemKind::PayloadTooLarge, "request body is too large"),
            422u16 => (ApiProblemKind::Validation, "request validation failed"),
            425u16 => (
                ApiProblemKind::InProgress,
                "matching request is still in progress",
            ),
            428u16 => (
                ApiProblemKind::PreconditionRequired,
                "request precondition is required",
            ),
            429u16 => (ApiProblemKind::RateLimited, "request rate limit exceeded"),
            500u16..=599u16 => (ApiProblemKind::Internal, "internal server error"),
            _ => (ApiProblemKind::RequestFailed, "request failed"),
        };
        Self {
            detail: ApiProblemDetail::try_from(detail.to_owned()).unwrap_or_default(),
            kind,
            request_id: None,
            status,
            violations: Vec::new(),
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
