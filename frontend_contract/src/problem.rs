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
            400u16 => (ApiProblemKind::InvalidRequest, str_constants::expr::S_1439),
            401u16 => (ApiProblemKind::Authentication, str_constants::expr::S_0975),
            403u16 => (ApiProblemKind::Authorization, str_constants::expr::S_0976),
            404u16 => (ApiProblemKind::NotFound, str_constants::expr::S_1671),
            405u16 => (
                ApiProblemKind::MethodNotAllowed,
                str_constants::expr::S_1515,
            ),
            409u16 => (ApiProblemKind::Conflict, str_constants::expr::S_1673),
            412u16 => (ApiProblemKind::Precondition, str_constants::expr::S_1672),
            413u16 => (ApiProblemKind::PayloadTooLarge, str_constants::expr::S_1657),
            422u16 => (ApiProblemKind::Validation, str_constants::expr::S_1664),
            425u16 => (ApiProblemKind::InProgress, str_constants::expr::S_1508),
            428u16 => (
                ApiProblemKind::PreconditionRequired,
                str_constants::expr::S_1660,
            ),
            429u16 => (ApiProblemKind::RateLimited, str_constants::expr::S_1661),
            500u16..=599u16 => (ApiProblemKind::Internal, str_constants::expr::S_1433),
            _ => (ApiProblemKind::RequestFailed, str_constants::expr::S_1658),
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
