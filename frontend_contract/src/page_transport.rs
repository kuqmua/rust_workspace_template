#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, PartialEq, Eq)]
pub struct PageContract {
    actions: super::ActionContracts,
    fields: super::FieldContracts,
    path: super::ContractStr,
    routes: super::RouteContracts,
    title: super::ContractStr,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, PartialEq, Eq, newtype::AsRefTarget,
)]
pub struct TransportBody(
    bounded_types::domain_types::vector::BoundedVec<
        u8,
        0,
        { super::FRONTEND_CONTRACT_BODY_MAX_BYTES },
    >,
);
impl TryFrom<Vec<u8>> for TransportBody {
    type Error = super::FrontendContractBodyError;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        bounded_types::domain_types::vector::BoundedVec::try_from(value)
            .map(Self)
            .map_err(super::FrontendContractBodyError::from)
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, PartialEq, Eq)]
pub struct TransportRequest {
    body: TransportBody,
    path: TransportPath,
    route: super::RouteContract,
    idempotency_key: Option<TransportIdempotencyKey>,
    if_match: Option<TransportIfMatch>,
}
impl TransportRequest {
    #[must_use]
    pub const fn new(
        body: TransportBody,
        path: TransportPath,
        route: super::RouteContract,
    ) -> Self {
        Self {
            body,
            path,
            route,
            idempotency_key: None,
            if_match: None,
        }
    }
    #[must_use]
    pub const fn body(&self) -> &TransportBody {
        &self.body
    }
    #[must_use]
    pub const fn path(&self) -> &TransportPath {
        &self.path
    }
    #[must_use]
    pub const fn route(&self) -> super::RouteContract {
        self.route
    }
    #[must_use]
    pub const fn idempotency_key(&self) -> Option<&TransportIdempotencyKey> {
        self.idempotency_key.as_ref()
    }
    #[must_use]
    pub const fn if_match(&self) -> Option<&TransportIfMatch> {
        self.if_match.as_ref()
    }
    #[must_use]
    pub fn with_idempotency_key(mut self, value: TransportIdempotencyKey) -> Self {
        self.idempotency_key = Some(value);
        self
    }
    #[must_use]
    pub fn with_if_match(mut self, value: TransportIfMatch) -> Self {
        self.if_match = Some(value);
        self
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    PartialEq,
    Eq,
    newtype::AsRefStr,
    newtype::BoundedString,
)]
#[bounded_string(max = 255usize, min = constants_usize::ONE)]
pub struct TransportIdempotencyKey(String);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    PartialEq,
    Eq,
    newtype::AsRefStr,
    newtype::BoundedString,
)]
#[bounded_string(max = 20usize, min = constants_usize::ONE)]
pub struct TransportIfMatch(String);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    newtype::AsRefStr,
    newtype::BoundedString,
)]
#[bounded_string(max = constants_usize::VALUE_8_192)]
pub struct TransportPath(String);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    newtype::Display,
    newtype::IntoInnerFrom,
    newtype::TryFrom,
)]
#[try_from(
    error = super::HttpStatusTryFromU16Error,
    validator = TransportStatus::validate
)]
pub struct TransportStatus(u16);
impl From<super::KnownHttpStatus> for TransportStatus {
    fn from(value: super::KnownHttpStatus) -> Self {
        Self(value.get())
    }
}
impl TransportStatus {
    #[allow(clippy::single_call_fn, clippy::trivially_copy_pass_by_ref)] // derive-generated TryFrom owns the single call and borrows the inner value
    fn validate(value: &u16) -> Result<(), super::HttpStatusTryFromU16Error> {
        if (100u16..1_000u16).contains(value) {
            Ok(())
        } else {
            Err(super::HttpStatusTryFromU16Error)
        }
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    PartialEq,
    Eq,
    newtype::AsRefStr,
    newtype::BoundedString,
)]
#[bounded_string(max = 128usize, min = constants_usize::ONE)]
pub struct TransportRetryAfter(String);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, PartialEq, Eq)]
pub struct TransportResponse {
    body: TransportBody,
    retry_after: Option<TransportRetryAfter>,
    status: TransportStatus,
}
impl TransportResponse {
    #[must_use]
    pub const fn new(body: TransportBody, status: TransportStatus) -> Self {
        Self {
            body,
            retry_after: None,
            status,
        }
    }
    #[must_use]
    pub fn with_retry_after(mut self, retry_after: Option<TransportRetryAfter>) -> Self {
        self.retry_after = retry_after;
        self
    }
    #[must_use]
    pub const fn body(&self) -> &TransportBody {
        &self.body
    }
    #[must_use]
    pub const fn status(&self) -> TransportStatus {
        self.status
    }
    #[must_use]
    pub const fn retry_after(&self) -> Option<&TransportRetryAfter> {
        self.retry_after.as_ref()
    }
    pub fn success_body(&self, expected: TransportStatus) -> Result<&TransportBody, ClientError> {
        if self.status == expected {
            Ok(&self.body)
        } else {
            Err(decode_api_problem(&self.body).map_or(
                ClientError::Status {
                    actual: self.status,
                    expected,
                },
                ClientError::Problem,
            ))
        }
    }
}
#[must_use]
pub fn decode_api_problem(body: &TransportBody) -> Option<super::ApiProblem> {
    serde_json::from_slice(body.as_ref()).ok()
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    newtype::Display,
    newtype::FromInner,
)]
pub struct TransportError(to_err_string::domain_types::ErrorText);
impl TryFrom<String> for TransportError {
    type Error = to_err_string::domain_types::ErrorTextTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        to_err_string::domain_types::ErrorText::try_from(value).map(Self)
    }
}
pub trait Transport {
    fn send(
        &self,
        request: TransportRequest,
    ) -> impl Future<Output = Result<TransportResponse, TransportError>> + '_;
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, PartialEq, Eq)]
pub enum ClientError {
    Decode(super::FormValueError),
    Encode(super::FormValueError),
    Problem(super::ApiProblem),
    Status {
        actual: TransportStatus,
        expected: TransportStatus,
    },
    Transport(TransportError),
    UnexpectedResponse,
}
impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Decode(value) => write!(f, "failed to decode response: {value}"),
            Self::Encode(value) => write!(f, "failed to encode request: {value}"),
            Self::Problem(value) => value.detail().as_ref().fmt(f),
            Self::Status { actual, expected } => {
                write!(f, "expected HTTP {expected}, received HTTP {actual}")
            }
            Self::Transport(value) => write!(f, "transport failed: {value}"),
            Self::UnexpectedResponse => {
                f.write_str(constants_str::SERVER_RETURNED_AN_ERROR_RESPONSE)
            }
        }
    }
}
impl PageContract {
    #[must_use]
    pub const fn new(
        actions: super::ActionContracts,
        fields: super::FieldContracts,
        path: super::ContractStr,
        routes: super::RouteContracts,
        title: super::ContractStr,
    ) -> Self {
        Self {
            actions,
            fields,
            path,
            routes,
            title,
        }
    }
    #[must_use]
    pub const fn actions(&self) -> &super::ActionContracts {
        &self.actions
    }
    #[must_use]
    pub const fn fields(&self) -> &super::FieldContracts {
        &self.fields
    }
    #[must_use]
    pub const fn path(&self) -> super::ContractStr {
        self.path
    }
    #[must_use]
    pub const fn routes(&self) -> &super::RouteContracts {
        &self.routes
    }
    #[must_use]
    pub const fn title(&self) -> super::ContractStr {
        self.title
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn transport_body_enforces_shared_limit() {
        let oversized = vec![
            constants_u8::ZERO;
            crate::domain_types::FRONTEND_CONTRACT_BODY_MAX_BYTES
                + constants_usize::ONE
        ];
        assert_eq!(
            super::TransportBody::try_from(oversized),
            Err(crate::domain_types::FrontendContractBodyError),
        );
    }
}
