#![allow(clippy::arbitrary_source_item_ordering)] // contract implementations keep constructors before accessors and fluent modifiers
const FRONTEND_CONTRACT_BODY_MAX_BYTES: usize = 16_777_216usize;
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("frontend contract body exceeds its maximum byte length")]
pub struct FrontendContractBodyError;
mod auth_session_keep_alive;
mod json_snapshot;
mod openapi_validation;
mod problem;
mod route;
mod route_contract_validation;
mod route_coverage;
mod url_builder;
pub use auth_session_keep_alive::{
    AuthSessionKeepAlive, AuthSessionKeepAliveDecision, AuthSessionKeepAliveError,
    AuthSessionPresence, AuthSessionRefreshOutcome, StdAuthSessionInstant,
    StdAuthSessionRefreshInterval,
};
pub use frontend_contract_macros::{
    PageCatalog, RouteCatalog, RouteFamily, TypedRoute, route_openapi, route_registry,
};
pub use json_snapshot::{
    JsonContractSnapshot, JsonContractSnapshotError, JsonSnapshotDynamicFieldRef,
    canonical_json_contract_snapshot,
};
pub use openapi_validation::{
    OpenApiContractText, OpenApiContractTextError, OpenApiContractTextTryFromStringError,
    OpenApiOperationExpectation, OpenApiOperationValidationError, OpenApiPayloadValidationError,
    OpenApiResponseStatus, OpenApiSchemaMismatch, OpenApiSecurityExpectation,
    OpenApiValidationError, RuntimeRoutesRef, SerdeJsonOpenApiSerializationError,
    validate_openapi_contract, validate_openapi_json_payload, validate_openapi_operations,
};
pub use problem::{
    ApiProblem, ApiProblemDetail, ApiProblemField, ApiProblemKind, ApiProblemRequestId,
    ApiProblemStatus, ApiProblemViolation,
};
pub use route::{
    AuthenticatedTransport, CoveredRoute, OpenApiSecuritySchemeRef, ParameterizedRoute,
    ParameterizedRoutePath, ParameterizedRoutePathTryFromStringError, PublicTransport,
    RouteBodyLimit, RouteCoverageDescriptors, RouteFamily, RouteMetadata, RouteMetadataList,
    RouteMethod, RouteRequest, RouteResponse, RouteSchemaContract, RouteSchemaContracts,
    RouteTransport, TypedRoute, UtoipaOpenApiPathParameter, UtoipaOpenApiRouteSchema,
    apply_openapi_error_contract, apply_openapi_path_parameter_contract,
    apply_openapi_security_contract, apply_openapi_success_contract, client_request,
    client_route_metadata, openapi_route_metadata, server_response, server_route_metadata,
    typed_parameterized_route_path, typed_route_path,
};
pub use route_contract_validation::{
    HttpContractBody, HttpContractBodyKind, HttpContractExpectation, HttpContractMismatch,
    HttpContractObservation, HttpContractStatus, RouteContractMismatch, RouteContractMismatches,
    run_http_contract_fixture, validate_route_contract_metadata, validate_typed_route_contract,
};
pub use route_coverage::{
    AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS,
    AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS, PUBLIC_MUTATING_ROUTE_COVERAGE_OBLIGATIONS,
    PUBLIC_READ_ROUTE_COVERAGE_OBLIGATIONS, RouteAccess, RouteCoverageDescriptor,
    RouteCoverageError, RouteCoverageEvidence, RouteCoverageObligation, RouteDatabaseUsage,
    RouteJsonBodyUsage, RouteMutation, RouteResponseKind, RouteTestCapabilities,
    RouteTestCategories, RouteTestCategory, missing_required_test_categories,
    required_test_categories, validate_route_coverage,
};
pub use url_builder::{ApiUrl, ApiUrlBuildError, ApiUrlPathSegmentRef, ApiUrlQueryComponentRef};
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, newtype::AsRefStr, newtype::Display, newtype::FromInner,
)]
pub struct ContractStr(&'static str);
impl From<ContractStr> for String {
    fn from(value: ContractStr) -> Self {
        Self::from(value.0)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputKind {
    Checkbox,
    Date,
    DateTime,
    Number,
    Text,
    Time,
    Uuid,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValueFormat {
    Bool,
    Bytes,
    Date,
    DateTime,
    Float32,
    Float64,
    Inet,
    Int16,
    Int32,
    Int64,
    Interval,
    Mac,
    Range,
    Text,
    Time,
    Timestamp,
    TimestampTz,
    Uuid,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nullability {
    NonNullable,
    Nullable,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CapabilitySupport {
    Supported,
    Unsupported,
}
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize, utoipa::ToSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum FilterOperation {
    AdjacentWithRange,
    Before,
    Between,
    CurrentDate,
    CurrentTime,
    CurrentTimestamp,
    Eq,
    EqToEncodedStringRepresentation,
    ExcludedUpperBound,
    FindRangesThatFullyContainTheGivenRange,
    FindRangesWithinGivenRange,
    GreaterThan,
    GreaterThanCurrentDate,
    GreaterThanCurrentTime,
    GreaterThanCurrentTimestamp,
    GreaterThanExcludedUpperBound,
    GreaterThanIncludedLowerBound,
    In,
    IncludedLowerBound,
    OverlapWithRange,
    RangeLen,
    Regex,
    StrictlyToLeftOfRange,
    StrictlyToRightOfRange,
}
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize, utoipa::ToSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum FilterValueShape {
    EncodedText,
    List,
    None,
    Range,
    Regex,
    Scalar,
}
impl FilterOperation {
    #[must_use]
    pub const fn value_shape(self) -> FilterValueShape {
        match self {
            Self::Between => FilterValueShape::Range,
            Self::CurrentDate
            | Self::CurrentTime
            | Self::CurrentTimestamp
            | Self::GreaterThanCurrentDate
            | Self::GreaterThanCurrentTime
            | Self::GreaterThanCurrentTimestamp => FilterValueShape::None,
            Self::EqToEncodedStringRepresentation => FilterValueShape::EncodedText,
            Self::In => FilterValueShape::List,
            Self::Regex => FilterValueShape::Regex,
            Self::AdjacentWithRange
            | Self::Before
            | Self::Eq
            | Self::ExcludedUpperBound
            | Self::FindRangesThatFullyContainTheGivenRange
            | Self::FindRangesWithinGivenRange
            | Self::GreaterThan
            | Self::GreaterThanExcludedUpperBound
            | Self::GreaterThanIncludedLowerBound
            | Self::IncludedLowerBound
            | Self::OverlapWithRange
            | Self::RangeLen
            | Self::StrictlyToLeftOfRange
            | Self::StrictlyToRightOfRange => FilterValueShape::Scalar,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, newtype::FromInner)]
pub struct FilterContracts(&'static [FilterOperation]);
impl AsRef<[FilterOperation]> for FilterContracts {
    fn as_ref(&self) -> &[FilterOperation] {
        self.0
    }
}
pub trait HasFilterContracts {
    const FILTER_CONTRACTS: &'static [FilterOperation];
    #[must_use]
    fn filter_contracts() -> FilterContracts {
        FilterContracts::from(Self::FILTER_CONTRACTS)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputStep {
    Any,
    Decimal,
    Integer,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NumericBound {
    None,
    Inclusive(ContractI64),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, newtype::FromInner)]
pub struct ContractI64(i64);
impl ContractI64 {
    #[must_use]
    pub fn i16_max() -> Self {
        Self::from(32_767i64)
    }
    #[must_use]
    pub fn i16_min() -> Self {
        Self::from(-32_768i64)
    }
    #[must_use]
    pub fn i32_max() -> Self {
        Self::from(2_147_483_647i64)
    }
    #[must_use]
    pub fn i32_min() -> Self {
        Self::from(-2_147_483_648i64)
    }
    #[must_use]
    pub fn max() -> Self {
        Self::from(i64::MAX)
    }
    #[must_use]
    pub fn min() -> Self {
        Self::from(i64::MIN)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValueExample {
    Boolean,
    Date,
    DateTime,
    Decimal,
    Integer,
    None,
    Text,
    Time,
    Uuid,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TypeContract {
    example: ValueExample,
    format: ValueFormat,
    input_kind: InputKind,
    maximum: NumericBound,
    minimum: NumericBound,
    nullability: Nullability,
    step: InputStep,
}
impl TypeContract {
    #[must_use]
    pub const fn new(input_kind: InputKind, format: ValueFormat, nullability: Nullability) -> Self {
        Self {
            example: ValueExample::None,
            format,
            input_kind,
            maximum: NumericBound::None,
            minimum: NumericBound::None,
            nullability,
            step: InputStep::Any,
        }
    }
    #[must_use]
    pub const fn example(self) -> ValueExample {
        self.example
    }
    #[must_use]
    pub const fn format(self) -> ValueFormat {
        self.format
    }
    #[must_use]
    pub const fn input_kind(self) -> InputKind {
        self.input_kind
    }
    #[must_use]
    pub const fn maximum(self) -> NumericBound {
        self.maximum
    }
    #[must_use]
    pub const fn minimum(self) -> NumericBound {
        self.minimum
    }
    #[must_use]
    pub const fn nullability(self) -> Nullability {
        self.nullability
    }
    #[must_use]
    pub const fn step(self) -> InputStep {
        self.step
    }
    #[must_use]
    pub const fn supports_filtering(self) -> CapabilitySupport {
        if matches!(
            self.format,
            ValueFormat::Bytes | ValueFormat::Interval | ValueFormat::Range
        ) {
            CapabilitySupport::Unsupported
        } else {
            CapabilitySupport::Supported
        }
    }
    #[must_use]
    pub const fn supports_sorting(self) -> CapabilitySupport {
        if matches!(self.format, ValueFormat::Bytes | ValueFormat::Range) {
            CapabilitySupport::Unsupported
        } else {
            CapabilitySupport::Supported
        }
    }
    #[must_use]
    pub const fn with_example(mut self, value: ValueExample) -> Self {
        self.example = value;
        self
    }
    #[must_use]
    pub const fn with_maximum(mut self, value: NumericBound) -> Self {
        self.maximum = value;
        self
    }
    #[must_use]
    pub const fn with_minimum(mut self, value: NumericBound) -> Self {
        self.minimum = value;
        self
    }
    #[must_use]
    pub const fn with_step(mut self, value: InputStep) -> Self {
        self.step = value;
        self
    }
}
pub trait HasTypeContract {
    fn type_contract() -> TypeContract;
}
#[derive(Clone, Debug, Default, PartialEq, Eq, newtype::AsRefStr, newtype::BoundedString)]
#[bounded_string(max = 1_048_576usize)]
pub struct FormValue(String);
#[derive(Clone, Copy, Debug, PartialEq, Eq, newtype::AsRefInner, newtype::FromInner)]
pub struct FormValueRef<'value_lt>(&'value_lt str);
#[derive(Clone, Copy, Debug, PartialEq, Eq, newtype::AsRefStr, newtype::FromInner)]
pub struct FormFieldNameRef<'field_lt>(&'field_lt str);
#[derive(Clone, Debug, Default, PartialEq, Eq, newtype::BoundedString, newtype::Display)]
#[bounded_string(max = 65536usize)]
pub struct FormValueError(String);
#[derive(Clone, Debug, PartialEq, Eq, newtype::AsRefStr, newtype::BoundedString)]
#[bounded_string(max = 1_048_576usize)]
pub struct FilterWireJson(String);
pub trait FormValueContract: Sized {
    fn format_form_value(&self) -> Result<FormValue, FormValueError>;
    fn parse_form_value(value: FormValueRef<'_>) -> Result<Self, FormValueError>;
}
pub trait FilterFormValueContract {
    fn parse_filter_form_value(value: FormValueRef<'_>) -> Result<FilterWireJson, FormValueError>;
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FormFieldError {
    error: FormValueError,
    field: ContractStr,
}
impl FormFieldError {
    #[must_use]
    pub const fn new(error: FormValueError, field: ContractStr) -> Self {
        Self { error, field }
    }
    #[must_use]
    pub const fn error(&self) -> &FormValueError {
        &self.error
    }
    #[must_use]
    pub const fn field(&self) -> ContractStr {
        self.field
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldCapability {
    Disabled,
    Enabled,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrimaryKeyKind {
    NonPrimary,
    Primary,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, newtype::FromInner)]
pub struct FieldOrder(usize);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldVisibility {
    Hidden,
    Visible,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldPlaceholder {
    None,
    Value(ContractStr),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldContract {
    creatable: FieldCapability,
    filterable: FieldCapability,
    filters: FilterContracts,
    label: ContractStr,
    name: ContractStr,
    order: FieldOrder,
    placeholder: FieldPlaceholder,
    primary_key: PrimaryKeyKind,
    readable: FieldCapability,
    sortable: FieldCapability,
    type_contract: TypeContract,
    updatable: FieldCapability,
    visibility: FieldVisibility,
}
#[derive(Clone, Debug, PartialEq, Eq, newtype::AsRefTarget, newtype::FromInner)]
pub struct FieldContracts(Vec<FieldContract>);
const EMPTY_FILTER_CONTRACTS: &[FilterOperation] = &[];
impl FieldContract {
    #[must_use]
    pub fn new(name: ContractStr, label: ContractStr, type_contract: TypeContract) -> Self {
        Self {
            creatable: FieldCapability::Disabled,
            filterable: FieldCapability::Disabled,
            filters: FilterContracts::from(EMPTY_FILTER_CONTRACTS),
            label,
            name,
            order: FieldOrder::from(0usize),
            placeholder: FieldPlaceholder::None,
            primary_key: PrimaryKeyKind::NonPrimary,
            readable: FieldCapability::Disabled,
            sortable: FieldCapability::Disabled,
            type_contract,
            updatable: FieldCapability::Disabled,
            visibility: FieldVisibility::Visible,
        }
    }
    #[must_use]
    pub const fn creatable(self) -> FieldCapability {
        self.creatable
    }
    #[must_use]
    pub const fn label(self) -> ContractStr {
        self.label
    }
    #[must_use]
    pub const fn filterable(self) -> FieldCapability {
        self.filterable
    }
    #[must_use]
    pub fn filters(&self) -> &[FilterOperation] {
        self.filters.as_ref()
    }
    #[must_use]
    pub const fn name(self) -> ContractStr {
        self.name
    }
    #[must_use]
    pub const fn order(self) -> FieldOrder {
        self.order
    }
    #[must_use]
    pub const fn placeholder(self) -> FieldPlaceholder {
        self.placeholder
    }
    #[must_use]
    pub const fn primary_key(self) -> PrimaryKeyKind {
        self.primary_key
    }
    #[must_use]
    pub const fn readable(self) -> FieldCapability {
        self.readable
    }
    #[must_use]
    pub const fn sortable(self) -> FieldCapability {
        self.sortable
    }
    #[must_use]
    pub const fn type_contract(self) -> TypeContract {
        self.type_contract
    }
    #[must_use]
    pub const fn updatable(self) -> FieldCapability {
        self.updatable
    }
    #[must_use]
    pub const fn visibility(self) -> FieldVisibility {
        self.visibility
    }
    #[must_use]
    pub const fn with_creatable(mut self, value: FieldCapability) -> Self {
        self.creatable = value;
        self
    }
    #[must_use]
    pub const fn with_filterable(mut self, value: FieldCapability) -> Self {
        self.filterable = value;
        self
    }
    #[must_use]
    pub const fn with_filters(mut self, value: FilterContracts) -> Self {
        self.filters = value;
        self
    }
    #[must_use]
    pub const fn with_order(mut self, value: FieldOrder) -> Self {
        self.order = value;
        self
    }
    #[must_use]
    pub const fn with_placeholder(mut self, value: FieldPlaceholder) -> Self {
        self.placeholder = value;
        self
    }
    #[must_use]
    pub const fn with_primary_key(mut self, value: PrimaryKeyKind) -> Self {
        self.primary_key = value;
        self
    }
    #[must_use]
    pub const fn with_readable(mut self, value: FieldCapability) -> Self {
        self.readable = value;
        self
    }
    #[must_use]
    pub const fn with_sortable(mut self, value: FieldCapability) -> Self {
        self.sortable = value;
        self
    }
    #[must_use]
    pub const fn with_updatable(mut self, value: FieldCapability) -> Self {
        self.updatable = value;
        self
    }
    #[must_use]
    pub const fn with_visibility(mut self, value: FieldVisibility) -> Self {
        self.visibility = value;
        self
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HttpMethod {
    Connect,
    Delete,
    Get,
    Head,
    Options,
    Patch,
    Post,
    Put,
    Trace,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SuccessStatus {
    Code200,
    Code201,
    Code204,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RouteErrorStatus {
    Authentication,
    Authorization,
    Conflict,
    Internal,
    RateLimited,
    Validation,
}
impl RouteErrorStatus {
    #[must_use]
    pub fn transport_status(self) -> TransportStatus {
        TransportStatus::from(match self {
            Self::Authentication => 401u16,
            Self::Authorization => 403u16,
            Self::Conflict => 409u16,
            Self::Internal => 500u16,
            Self::RateLimited => 429u16,
            Self::Validation => 422u16,
        })
    }
}
pub const PUBLIC_AUTH_ROUTE_ERROR_STATUSES: &[RouteErrorStatus] = &[
    RouteErrorStatus::Authentication,
    RouteErrorStatus::RateLimited,
    RouteErrorStatus::Internal,
];
pub const PUBLIC_READ_ROUTE_ERROR_STATUSES: &[RouteErrorStatus] =
    &[RouteErrorStatus::Internal, RouteErrorStatus::RateLimited];
pub const PUBLIC_MUTATING_ROUTE_ERROR_STATUSES: &[RouteErrorStatus] = &[
    RouteErrorStatus::Validation,
    RouteErrorStatus::RateLimited,
    RouteErrorStatus::Internal,
];
pub const PUBLIC_REFRESH_ROUTE_ERROR_STATUSES: &[RouteErrorStatus] = &[
    RouteErrorStatus::Authentication,
    RouteErrorStatus::RateLimited,
    RouteErrorStatus::Internal,
];
pub const AUTHENTICATED_READ_ROUTE_ERROR_STATUSES: &[RouteErrorStatus] = &[
    RouteErrorStatus::Authentication,
    RouteErrorStatus::RateLimited,
    RouteErrorStatus::Internal,
];
pub const AUTHORIZED_READ_ROUTE_ERROR_STATUSES: &[RouteErrorStatus] = &[
    RouteErrorStatus::Authentication,
    RouteErrorStatus::Authorization,
    RouteErrorStatus::RateLimited,
    RouteErrorStatus::Internal,
];
pub const AUTHORIZED_VALIDATED_READ_ROUTE_ERROR_STATUSES: &[RouteErrorStatus] = &[
    RouteErrorStatus::Authentication,
    RouteErrorStatus::Authorization,
    RouteErrorStatus::Validation,
    RouteErrorStatus::RateLimited,
    RouteErrorStatus::Internal,
];
pub const AUTHENTICATED_MUTATING_ROUTE_ERROR_STATUSES: &[RouteErrorStatus] = &[
    RouteErrorStatus::Authentication,
    RouteErrorStatus::Authorization,
    RouteErrorStatus::RateLimited,
    RouteErrorStatus::Internal,
];
pub const AUTHORIZED_MUTATING_ROUTE_ERROR_STATUSES: &[RouteErrorStatus] = &[
    RouteErrorStatus::Authentication,
    RouteErrorStatus::Authorization,
    RouteErrorStatus::Conflict,
    RouteErrorStatus::Validation,
    RouteErrorStatus::RateLimited,
    RouteErrorStatus::Internal,
];
pub const AUTHORIZED_DELETE_ROUTE_ERROR_STATUSES: &[RouteErrorStatus] = &[
    RouteErrorStatus::Authentication,
    RouteErrorStatus::Authorization,
    RouteErrorStatus::Conflict,
    RouteErrorStatus::RateLimited,
    RouteErrorStatus::Internal,
];
impl SuccessStatus {
    #[must_use]
    pub fn transport_status(self) -> TransportStatus {
        TransportStatus::from(match self {
            Self::Code200 => 200u16,
            Self::Code201 => 201u16,
            Self::Code204 => 204u16,
        })
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuthenticationRequirement {
    Authenticated,
    Permission(ContractStr),
    Public,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MutationKind {
    ReadOnly,
    Mutating,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OperationKind {
    CreateMany,
    CreateOne,
    DeleteMany,
    DeleteOne,
    ReadMany,
    ReadOne,
    UpdateMany,
    UpdateOne,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConfirmationRequirement {
    NotRequired,
    Required,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActionContract {
    confirmation: ConfirmationRequirement,
    operation: OperationKind,
    route: RouteContract,
}
#[derive(Clone, Debug, PartialEq, Eq, newtype::AsRefTarget, newtype::FromInner)]
pub struct ActionContracts(Vec<ActionContract>);
impl ActionContract {
    #[must_use]
    pub const fn new(operation: OperationKind, route: RouteContract) -> Self {
        Self {
            confirmation: ConfirmationRequirement::NotRequired,
            operation,
            route,
        }
    }
    #[must_use]
    pub const fn confirmation(self) -> ConfirmationRequirement {
        self.confirmation
    }
    #[must_use]
    pub const fn operation(self) -> OperationKind {
        self.operation
    }
    #[must_use]
    pub const fn route(self) -> RouteContract {
        self.route
    }
    #[must_use]
    pub const fn with_confirmation(mut self, value: ConfirmationRequirement) -> Self {
        self.confirmation = value;
        self
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RouteContract {
    authentication: AuthenticationRequirement,
    method: HttpMethod,
    mutation: MutationKind,
    path: ContractStr,
    success_status: SuccessStatus,
}
#[derive(Clone, Debug, PartialEq, Eq, newtype::AsRefTarget, newtype::FromInner)]
pub struct RouteContracts(Vec<RouteContract>);
impl RouteContract {
    #[must_use]
    pub const fn new(
        authentication: AuthenticationRequirement,
        method: HttpMethod,
        mutation: MutationKind,
        path: ContractStr,
        success_status: SuccessStatus,
    ) -> Self {
        Self {
            authentication,
            method,
            mutation,
            path,
            success_status,
        }
    }
    #[must_use]
    pub const fn authentication(self) -> AuthenticationRequirement {
        self.authentication
    }
    #[must_use]
    pub const fn method(self) -> HttpMethod {
        self.method
    }
    #[must_use]
    pub const fn mutation(self) -> MutationKind {
        self.mutation
    }
    #[must_use]
    pub const fn path(self) -> ContractStr {
        self.path
    }
    #[must_use]
    pub const fn success_status(self) -> SuccessStatus {
        self.success_status
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PageContract {
    actions: ActionContracts,
    fields: FieldContracts,
    path: ContractStr,
    routes: RouteContracts,
    title: ContractStr,
}
#[derive(Clone, Debug, PartialEq, Eq, newtype::AsRefTarget)]
pub struct TransportBody(Vec<u8>);
impl TryFrom<Vec<u8>> for TransportBody {
    type Error = FrontendContractBodyError;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if value.len() > FRONTEND_CONTRACT_BODY_MAX_BYTES {
            Err(FrontendContractBodyError)
        } else {
            Ok(Self(value))
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransportRequest {
    body: TransportBody,
    path: TransportPath,
    route: RouteContract,
    idempotency_key: Option<TransportIdempotencyKey>,
    if_match: Option<TransportIfMatch>,
}
impl TransportRequest {
    #[must_use]
    pub const fn new(body: TransportBody, path: TransportPath, route: RouteContract) -> Self {
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
    pub const fn route(&self) -> RouteContract {
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
#[derive(Clone, Debug, PartialEq, Eq, newtype::AsRefStr, newtype::BoundedString)]
#[bounded_string(max = 255usize, min = 1usize)]
pub struct TransportIdempotencyKey(String);
#[derive(Clone, Debug, PartialEq, Eq, newtype::AsRefStr, newtype::BoundedString)]
#[bounded_string(max = 20usize, min = 1usize)]
pub struct TransportIfMatch(String);
#[derive(Clone, Debug, Default, PartialEq, Eq, newtype::AsRefStr, newtype::BoundedString)]
#[bounded_string(max = 8192usize)]
pub struct TransportPath(String);
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, newtype::Display, newtype::FromInner, newtype::IntoInnerFrom,
)]
pub struct TransportStatus(u16);
#[derive(Clone, Debug, PartialEq, Eq, newtype::AsRefStr, newtype::BoundedString)]
#[bounded_string(max = 128usize, min = 1usize)]
pub struct TransportRetryAfter(String);
#[derive(Clone, Debug, PartialEq, Eq)]
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
pub fn decode_api_problem(body: &TransportBody) -> Option<ApiProblem> {
    serde_json::from_slice(body.as_ref()).ok()
}
#[derive(Clone, Debug, Default, PartialEq, Eq, newtype::BoundedString, newtype::Display)]
#[bounded_string(max = 65536usize)]
pub struct TransportError(String);
pub trait Transport {
    fn send(
        &self,
        request: TransportRequest,
    ) -> impl Future<Output = Result<TransportResponse, TransportError>> + '_;
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ClientError {
    Decode(FormValueError),
    Encode(FormValueError),
    Problem(ApiProblem),
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
                f.write_str(str_constants::SERVER_RETURNED_AN_ERROR_RESPONSE)
            }
        }
    }
}
impl PageContract {
    #[must_use]
    pub const fn new(
        actions: ActionContracts,
        fields: FieldContracts,
        path: ContractStr,
        routes: RouteContracts,
        title: ContractStr,
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
    pub const fn actions(&self) -> &ActionContracts {
        &self.actions
    }
    #[must_use]
    pub const fn fields(&self) -> &FieldContracts {
        &self.fields
    }
    #[must_use]
    pub const fn path(&self) -> ContractStr {
        self.path
    }
    #[must_use]
    pub const fn routes(&self) -> &RouteContracts {
        &self.routes
    }
    #[must_use]
    pub const fn title(&self) -> ContractStr {
        self.title
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn contract_bodies_reject_values_above_shared_limit() {
        let oversized = vec![0u8; super::FRONTEND_CONTRACT_BODY_MAX_BYTES + 1usize];
        assert_eq!(
            super::TransportBody::try_from(oversized.clone()),
            Err(super::FrontendContractBodyError)
        );
        assert_eq!(
            super::HttpContractBody::try_from(oversized),
            Err(super::FrontendContractBodyError)
        );
    }
    #[allow(clippy::needless_for_each)] // iterator form follows the workspace ban on explicit for loops
    #[test]
    fn api_problem_status_mapping_is_stable_and_redacted() {
        let cases = [
            (401u16, super::ApiProblemKind::Authentication),
            (403u16, super::ApiProblemKind::Authorization),
            (404u16, super::ApiProblemKind::NotFound),
            (405u16, super::ApiProblemKind::MethodNotAllowed),
            (409u16, super::ApiProblemKind::Conflict),
            (412u16, super::ApiProblemKind::Precondition),
            (425u16, super::ApiProblemKind::InProgress),
            (428u16, super::ApiProblemKind::PreconditionRequired),
            (429u16, super::ApiProblemKind::RateLimited),
            (500u16, super::ApiProblemKind::Internal),
        ];
        cases.into_iter().for_each(|(status, expected_kind)| {
            let problem = super::ApiProblem::from_status(super::ApiProblemStatus::from(status));
            assert_eq!(problem.kind(), expected_kind);
            assert_eq!(u16::from(problem.status()), status);
            let serialized = serde_json::to_string(&problem).expect("f459312e");
            assert!(!serialized.contains("postgres://"));
            assert!(!serialized.contains("sqlx"));
            assert!(!serialized.contains("password"));
        });
    }
    #[test]
    fn contracts_preserve_typed_metadata() {
        let type_contract = super::TypeContract::new(
            super::InputKind::Number,
            super::ValueFormat::Int64,
            super::Nullability::NonNullable,
        )
        .with_minimum(super::NumericBound::Inclusive(super::ContractI64::from(1)))
        .with_step(super::InputStep::Integer);
        let field = super::FieldContract::new(
            super::ContractStr::from(str_constants::SQL_NAMES_ID),
            super::ContractStr::from(str_constants::ID),
            type_contract,
        )
        .with_primary_key(super::PrimaryKeyKind::Primary)
        .with_readable(super::FieldCapability::Enabled);
        assert_eq!(field.type_contract().input_kind(), super::InputKind::Number);
        assert_eq!(field.primary_key(), super::PrimaryKeyKind::Primary);
        assert_eq!(field.readable(), super::FieldCapability::Enabled);
    }
    #[test]
    fn route_contract_keeps_transport_policy_together() {
        let route = super::RouteContract::new(
            super::AuthenticationRequirement::Permission(super::ContractStr::from(
                str_constants::PERMISSION,
            )),
            super::HttpMethod::Patch,
            super::MutationKind::Mutating,
            super::ContractStr::from(str_constants::USERS_ID),
            super::SuccessStatus::Code204,
        );
        assert_eq!(route.mutation(), super::MutationKind::Mutating);
        assert_eq!(route.method(), super::HttpMethod::Patch);
        assert_eq!(route.path().as_ref(), "/users/{id}");
    }
    #[test]
    fn response_interpretation_uses_shared_success_and_problem_contract() {
        let problem = super::ApiProblem::from_status(super::ApiProblemStatus::from(401u16));
        let body = super::TransportBody::try_from(serde_json::to_vec(&problem).expect("f542a3cb"))
            .expect("864276f2");
        let response = super::TransportResponse::new(body, super::TransportStatus::from(401u16));
        let error = response
            .success_body(super::SuccessStatus::Code200.transport_status())
            .expect_err(str_constants::VALUE_5EEA7F90);
        assert!(matches!(
            error,
            super::ClientError::Problem(value)
                if value.kind() == super::ApiProblemKind::Authentication
        ));
        assert_eq!(
            u16::from(super::SuccessStatus::Code201.transport_status()),
            201u16
        );
    }
    #[test]
    fn transport_response_preserves_retry_after() {
        let response = super::TransportResponse::new(
            super::TransportBody::try_from(Vec::new()).expect("da32dc29"),
            super::TransportStatus::from(429u16),
        )
        .with_retry_after(Some(
            super::TransportRetryAfter::try_from(str_constants::TEST_VALUE_30.to_owned())
                .expect("9b6750d4"),
        ));
        assert_eq!(
            response.retry_after().map(AsRef::as_ref),
            Some(str_constants::TEST_VALUE_30)
        );
    }
}
