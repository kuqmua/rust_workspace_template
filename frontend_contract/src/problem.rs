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
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    newtype::IntoInnerFrom,
)]
#[serde(try_from = "u16")]
pub struct ApiProblemStatus(u16);
impl TryFrom<u16> for ApiProblemStatus {
    type Error = crate::HttpStatusTryFromU16Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if (100u16..1_000u16).contains(&value) {
            Ok(Self(value))
        } else {
            Err(crate::HttpStatusTryFromU16Error)
        }
    }
}
impl From<crate::KnownHttpStatus> for ApiProblemStatus {
    fn from(value: crate::KnownHttpStatus) -> Self {
        Self(value.get())
    }
}
#[derive(
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
#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
pub struct ApiProblemViolation {
    detail: ApiProblemDetail,
    field: ApiProblemField,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    newtype::TryFrom,
)]
#[try_from(validator = |value: &Vec<ApiProblemViolation>| {
    if value.len() > 128usize {
        Err(ApiProblemViolationsError)
    } else {
        Ok(())
    }
})]
#[serde(try_from = "Vec<ApiProblemViolation>")]
pub(crate) struct ApiProblemViolations(Vec<ApiProblemViolation>);
#[derive(Clone, Copy, Debug, Eq, PartialEq, newtype::DebugDisplay, newtype::Error)]
pub(crate) struct ApiProblemViolationsError;
#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
pub struct ApiProblem {
    detail: ApiProblemDetail,
    kind: ApiProblemKind,
    request_id: Option<ApiProblemRequestId>,
    status: ApiProblemStatus,
    #[schema(inline)]
    violations: ApiProblemViolations,
}
impl ApiProblem {
    #[must_use]
    pub fn from_status(status: ApiProblemStatus) -> Self {
        let (kind, detail) = match u16::from(status) {
            400u16 => (
                ApiProblemKind::InvalidRequest,
                str_constants::INVALID_REQUEST,
            ),
            401u16 => (
                ApiProblemKind::Authentication,
                str_constants::AUTHENTICATION_REQUIRED,
            ),
            403u16 => (
                ApiProblemKind::Authorization,
                str_constants::AUTHORIZATION_FAILED,
            ),
            404u16 => (ApiProblemKind::NotFound, str_constants::RESOURCE_NOT_FOUND),
            405u16 => (
                ApiProblemKind::MethodNotAllowed,
                str_constants::METHOD_NOT_ALLOWED,
            ),
            409u16 => (
                ApiProblemKind::Conflict,
                str_constants::RESOURCE_STATE_CONFLICT,
            ),
            412u16 => (
                ApiProblemKind::Precondition,
                str_constants::RESOURCE_PRECONDITION_FAILED,
            ),
            413u16 => (
                ApiProblemKind::PayloadTooLarge,
                str_constants::REQUEST_BODY_IS_TOO_LARGE,
            ),
            422u16 => (
                ApiProblemKind::Validation,
                str_constants::REQUEST_VALIDATION_FAILED,
            ),
            425u16 => (
                ApiProblemKind::InProgress,
                str_constants::MATCHING_REQUEST_IS_STILL_IN_PROGRESS,
            ),
            428u16 => (
                ApiProblemKind::PreconditionRequired,
                str_constants::REQUEST_PRECONDITION_IS_REQUIRED,
            ),
            429u16 => (
                ApiProblemKind::RateLimited,
                str_constants::REQUEST_RATE_LIMIT_EXCEEDED_ALT,
            ),
            500u16..=599u16 => (
                ApiProblemKind::Internal,
                str_constants::INTERNAL_SERVER_ERROR,
            ),
            _ => (ApiProblemKind::RequestFailed, str_constants::REQUEST_FAILED),
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

#[cfg(test)]
mod tests {
    #[test]
    fn problem_text_deserialization_uses_bounded_try_from() {
        let detail = serde_json::to_string(&"x".repeat(1_025usize)).expect("6e2db8a1");
        let request_id = serde_json::to_string(&"x".repeat(129usize)).expect("f289a40c");
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
        let serialized = serde_json::to_string(&vec![item; 129usize]).expect("a1010d3f");
        let _error =
            serde_json::from_str::<super::ApiProblemViolations>(&serialized).expect_err("a05e84a8");
    }
}
