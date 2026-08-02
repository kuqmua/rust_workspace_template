#![allow(clippy::arbitrary_source_item_ordering)] // contract implementations keep constructors before accessors and fluent modifiers
pub const FRONTEND_CONTRACT_BODY_MAX_BYTES: usize = 16_777_216usize;
#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("frontend contract body exceeds its maximum byte length")]
pub struct FrontendContractBodyError;
impl From<bounded_types::BoundedValueError> for FrontendContractBodyError {
    fn from(_value: bounded_types::BoundedValueError) -> Self {
        Self
    }
}
#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("{self:?}")]
pub struct HttpStatusTryFromU16Error;
#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq)]
pub enum KnownHttpStatus {
    BadRequest,
    Conflict,
    Created,
    Forbidden,
    InternalServerError,
    MethodNotAllowed,
    NoContent,
    NotFound,
    Ok,
    PayloadTooLarge,
    PreconditionFailed,
    PreconditionRequired,
    ServiceUnavailable,
    TooEarly,
    TooManyRequests,
    Unauthorized,
    UnprocessableEntity,
}
impl KnownHttpStatus {
    #[must_use]
    pub const fn get(self) -> u16 {
        match self {
            Self::BadRequest => 400u16,
            Self::Conflict => 409u16,
            Self::Created => 201u16,
            Self::Forbidden => 403u16,
            Self::InternalServerError => 500u16,
            Self::MethodNotAllowed => 405u16,
            Self::NoContent => 204u16,
            Self::NotFound => 404u16,
            Self::Ok => 200u16,
            Self::PayloadTooLarge => 413u16,
            Self::PreconditionFailed => 412u16,
            Self::PreconditionRequired => 428u16,
            Self::ServiceUnavailable => 503u16,
            Self::TooEarly => 425u16,
            Self::TooManyRequests => 429u16,
            Self::Unauthorized => 401u16,
            Self::UnprocessableEntity => 422u16,
        }
    }
}
mod auth_session_keep_alive;
mod client;
mod handler_contract;
mod problem;
mod route;
mod route_coverage;
mod url_builder;
pub use auth_session_keep_alive::{
    AuthSessionKeepAlive, AuthSessionKeepAliveDecision, AuthSessionKeepAliveError,
    AuthSessionPresence, AuthSessionRefreshOutcome, StdAuthSessionInstant,
    StdAuthSessionRefreshInterval,
};
pub use client::TypedClient;
pub use frontend_contract_macros::{
    ContractStructApi, PageCatalog, RouteCatalog, RouteFamily, TypedRoute, UnitEnumCatalog,
    UnitEnumIndex, api_operation_error, handler_registry, route_error, route_openapi,
    route_operation, route_registry,
};
#[cfg(not(target_arch = "wasm32"))]
pub use handler_contract::{AxumHandlerMethodRouter, handler_method_router};
pub use handler_contract::{HandlerContract, HandlerPath};
pub use problem::{
    ApiProblem, ApiProblemDetail, ApiProblemError, ApiProblemField, ApiProblemKind,
    ApiProblemRequestId, ApiProblemStatus, ApiProblemViolation,
};
pub use route::{
    AuthenticatedTransport, CoveredRoute, OpenApiSecuritySchemeRef, ParameterizedRoute,
    ParameterizedRoutePath, ParameterizedRoutePathTryFromStringError, PublicTransport,
    RouteBodyLimit, RouteCoverageDescriptors, RouteFamily, RouteInFamily, RouteMetadata,
    RouteMetadataList, RouteMethod, RouteRequest, RouteRequestBody, RouteResponse,
    RouteSchemaContract, RouteSchemaContracts, RouteTransport, TypedRoute,
    UtoipaOpenApiComponentsRefMut, UtoipaOpenApiPathParameter, UtoipaOpenApiRefMut,
    UtoipaOpenApiRouteSchema, apply_openapi_error_contract, apply_openapi_path_parameter_contract,
    apply_openapi_request_contract, apply_openapi_security_contract,
    apply_openapi_success_contract, client_request, client_route_metadata, openapi_route_metadata,
    register_openapi_route_schemas, register_openapi_schema, server_response,
    server_route_metadata, typed_parameterized_route_path, typed_route_path,
};
#[cfg(not(target_arch = "wasm32"))]
pub use route::{AxumMethodFilter, axum_method_filter};
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
    optml::Optml,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    newtype::AsRefStr,
    newtype::Display,
    newtype::FromInner,
)]
pub struct ContractStr(&'static str);
impl From<ContractStr> for String {
    fn from(value: ContractStr) -> Self {
        Self::from(value.0)
    }
}
#[derive(optml::Optml, Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputKind {
    Checkbox,
    Date,
    DateTime,
    Number,
    Text,
    Time,
    Uuid,
}
#[derive(optml::Optml, Clone, Copy, Debug, PartialEq, Eq)]
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
#[derive(optml::Optml, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nullability {
    NonNullable,
    Nullable,
}
#[derive(optml::Optml, Clone, Copy, Debug, PartialEq, Eq)]
pub enum CapabilitySupport {
    Supported,
    Unsupported,
}
#[derive(
    optml::Optml,
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
    optml::Optml,
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
#[derive(
    optml::Optml, Clone, Copy, Debug, PartialEq, Eq, newtype::AsRefInner, newtype::FromInner,
)]
pub struct FilterContracts(&'static [FilterOperation]);
pub trait HasFilterContracts {
    const FILTER_CONTRACTS: &'static [FilterOperation];
    #[must_use]
    fn filter_contracts() -> FilterContracts {
        FilterContracts::from(Self::FILTER_CONTRACTS)
    }
}
#[derive(optml::Optml, Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputStep {
    Any,
    Decimal,
    Integer,
}
#[derive(optml::Optml, Clone, Copy, Debug, PartialEq, Eq)]
pub enum NumericBound {
    None,
    Inclusive(ContractI64),
}
#[derive(optml::Optml, Clone, Copy, Debug, PartialEq, Eq, newtype::FromInner)]
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
#[derive(optml::Optml, Clone, Copy, Debug, PartialEq, Eq)]
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
#[derive(optml::Optml, Clone, Copy, Debug, PartialEq, Eq)]
pub struct TypeContract {
    maximum: NumericBound,
    minimum: NumericBound,
    example: ValueExample,
    format: ValueFormat,
    input_kind: InputKind,
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
#[derive(
    optml::Optml, Clone, Debug, Default, PartialEq, Eq, newtype::AsRefStr, newtype::BoundedString,
)]
#[bounded_string(max = 1_048_576usize)]
pub struct FormValue(String);
#[derive(
    optml::Optml, Clone, Copy, Debug, PartialEq, Eq, newtype::AsRefInner, newtype::FromInner,
)]
pub struct FormValueRef<'value_lt>(&'value_lt str);
#[derive(
    optml::Optml, Clone, Copy, Debug, PartialEq, Eq, newtype::AsRefStr, newtype::FromInner,
)]
pub struct FormFieldNameRef<'field_lt>(&'field_lt str);
#[derive(
    optml::Optml, Clone, Debug, Default, PartialEq, Eq, newtype::Display, newtype::FromInner,
)]
pub struct FormValueError(to_err_string::ErrorText);
impl TryFrom<String> for FormValueError {
    type Error = to_err_string::ErrorTextTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        to_err_string::ErrorText::try_from(value).map(Self)
    }
}
#[derive(optml::Optml, Clone, Debug, PartialEq, Eq, newtype::AsRefStr, newtype::BoundedString)]
#[bounded_string(max = 1_048_576usize)]
pub struct FilterWireJson(String);
pub trait FormValueContract: Sized {
    fn format_form_value(&self) -> Result<FormValue, FormValueError>;
    fn parse_form_value(value: FormValueRef<'_>) -> Result<Self, FormValueError>;
}
pub trait FilterFormValueContract {
    fn parse_filter_form_value(value: FormValueRef<'_>) -> Result<FilterWireJson, FormValueError>;
}
#[derive(optml::Optml, Clone, Debug, PartialEq, Eq)]
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
#[derive(optml::Optml, Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldCapability {
    Disabled,
    Enabled,
}
#[derive(optml::Optml, Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrimaryKeyKind {
    NonPrimary,
    Primary,
}
#[derive(optml::Optml, Clone, Copy, Debug, PartialEq, Eq, newtype::FromInner)]
pub struct FieldOrder(usize);
#[derive(optml::Optml, Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldVisibility {
    Hidden,
    Visible,
}
#[derive(optml::Optml, Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldPlaceholder {
    None,
    Value(ContractStr),
}
#[derive(optml::Optml, Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldContract {
    filters: FilterContracts,
    label: ContractStr,
    name: ContractStr,
    placeholder: FieldPlaceholder,
    type_contract: TypeContract,
    order: FieldOrder,
    creatable: FieldCapability,
    filterable: FieldCapability,
    primary_key: PrimaryKeyKind,
    readable: FieldCapability,
    sortable: FieldCapability,
    updatable: FieldCapability,
    visibility: FieldVisibility,
}
#[derive(optml::Optml, Clone, Debug, PartialEq, Eq, newtype::AsRefTarget, newtype::FromInner)]
pub struct FieldContracts(bounded_types::BoundedVec<FieldContract, 0, { usize::MAX }>);
impl From<Vec<FieldContract>> for FieldContracts {
    fn from(value: Vec<FieldContract>) -> Self {
        Self::from(bounded_types::BoundedVec::from_max_iter(value))
    }
}
impl FieldContracts {
    #[must_use]
    pub fn from_max_iter<Values>(values: Values) -> Self
    where
        Values: IntoIterator<Item = FieldContract>,
    {
        Self::from(bounded_types::BoundedVec::from_max_iter(values))
    }
}
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
#[derive(optml::Optml, Clone, Copy, Debug, PartialEq, Eq)]
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
#[derive(optml::Optml, Clone, Copy, Debug, PartialEq, Eq)]
pub enum SuccessStatus {
    Code200,
    Code201,
    Code204,
}
#[derive(optml::Optml, Clone, Copy, Debug, PartialEq, Eq)]
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
    pub fn transport_status(self) -> TransportStatus {
        match self {
            Self::Authentication => TransportStatus::from(KnownHttpStatus::Unauthorized),
            Self::Authorization => TransportStatus::from(KnownHttpStatus::Forbidden),
            Self::Conflict => TransportStatus::from(KnownHttpStatus::Conflict),
            Self::Internal => TransportStatus::from(KnownHttpStatus::InternalServerError),
            Self::MethodNotAllowed => TransportStatus::from(KnownHttpStatus::MethodNotAllowed),
            Self::PayloadTooLarge => TransportStatus::from(KnownHttpStatus::PayloadTooLarge),
            Self::RateLimited => TransportStatus::from(KnownHttpStatus::TooManyRequests),
            Self::ServiceUnavailable => TransportStatus::from(KnownHttpStatus::ServiceUnavailable),
            Self::Validation => TransportStatus::from(KnownHttpStatus::UnprocessableEntity),
        }
    }
}
pub const PUBLIC_AUTH_ROUTE_ERROR_STATUSES: &[RouteErrorStatus] = &[
    RouteErrorStatus::Authentication,
    RouteErrorStatus::PayloadTooLarge,
    RouteErrorStatus::RateLimited,
    RouteErrorStatus::Internal,
];
pub const PUBLIC_READ_ROUTE_ERROR_STATUSES: &[RouteErrorStatus] = &[
    RouteErrorStatus::Internal,
    RouteErrorStatus::PayloadTooLarge,
    RouteErrorStatus::RateLimited,
];
pub const PUBLIC_MUTATING_ROUTE_ERROR_STATUSES: &[RouteErrorStatus] = &[
    RouteErrorStatus::PayloadTooLarge,
    RouteErrorStatus::Validation,
    RouteErrorStatus::RateLimited,
    RouteErrorStatus::Internal,
];
pub const PUBLIC_REFRESH_ROUTE_ERROR_STATUSES: &[RouteErrorStatus] = &[
    RouteErrorStatus::Authentication,
    RouteErrorStatus::PayloadTooLarge,
    RouteErrorStatus::RateLimited,
    RouteErrorStatus::Internal,
];
pub const AUTHENTICATED_READ_ROUTE_ERROR_STATUSES: &[RouteErrorStatus] = &[
    RouteErrorStatus::Authentication,
    RouteErrorStatus::PayloadTooLarge,
    RouteErrorStatus::RateLimited,
    RouteErrorStatus::Internal,
];
pub const AUTHORIZED_READ_ROUTE_ERROR_STATUSES: &[RouteErrorStatus] = &[
    RouteErrorStatus::Authentication,
    RouteErrorStatus::Authorization,
    RouteErrorStatus::PayloadTooLarge,
    RouteErrorStatus::RateLimited,
    RouteErrorStatus::Internal,
];
pub const AUTHORIZED_VALIDATED_READ_ROUTE_ERROR_STATUSES: &[RouteErrorStatus] = &[
    RouteErrorStatus::Authentication,
    RouteErrorStatus::Authorization,
    RouteErrorStatus::PayloadTooLarge,
    RouteErrorStatus::Validation,
    RouteErrorStatus::RateLimited,
    RouteErrorStatus::Internal,
];
pub const AUTHENTICATED_MUTATING_ROUTE_ERROR_STATUSES: &[RouteErrorStatus] = &[
    RouteErrorStatus::Authentication,
    RouteErrorStatus::Authorization,
    RouteErrorStatus::PayloadTooLarge,
    RouteErrorStatus::RateLimited,
    RouteErrorStatus::Internal,
];
pub const AUTHORIZED_MUTATING_ROUTE_ERROR_STATUSES: &[RouteErrorStatus] = &[
    RouteErrorStatus::Authentication,
    RouteErrorStatus::Authorization,
    RouteErrorStatus::Conflict,
    RouteErrorStatus::PayloadTooLarge,
    RouteErrorStatus::Validation,
    RouteErrorStatus::RateLimited,
    RouteErrorStatus::Internal,
];
pub const AUTHORIZED_DELETE_ROUTE_ERROR_STATUSES: &[RouteErrorStatus] = &[
    RouteErrorStatus::Authentication,
    RouteErrorStatus::Authorization,
    RouteErrorStatus::Conflict,
    RouteErrorStatus::PayloadTooLarge,
    RouteErrorStatus::RateLimited,
    RouteErrorStatus::Internal,
];
#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq)]
pub enum RouteErrorPolicy {
    Authentication,
    Default,
    Delete,
    ValidatedRead,
}
impl RouteErrorPolicy {
    #[must_use]
    pub const fn statuses(
        self,
        authentication: AuthenticationRequirement,
        mutation: RouteMutation,
    ) -> &'static [RouteErrorStatus] {
        match self {
            Self::Authentication => PUBLIC_AUTH_ROUTE_ERROR_STATUSES,
            Self::Delete => AUTHORIZED_DELETE_ROUTE_ERROR_STATUSES,
            Self::ValidatedRead => AUTHORIZED_VALIDATED_READ_ROUTE_ERROR_STATUSES,
            Self::Default => match (authentication, mutation) {
                (AuthenticationRequirement::Public, RouteMutation::ReadOnly) => {
                    PUBLIC_READ_ROUTE_ERROR_STATUSES
                }
                (AuthenticationRequirement::Public, RouteMutation::Mutating) => {
                    PUBLIC_MUTATING_ROUTE_ERROR_STATUSES
                }
                (AuthenticationRequirement::Authenticated, RouteMutation::ReadOnly) => {
                    AUTHENTICATED_READ_ROUTE_ERROR_STATUSES
                }
                (AuthenticationRequirement::Authenticated, RouteMutation::Mutating) => {
                    AUTHENTICATED_MUTATING_ROUTE_ERROR_STATUSES
                }
                (AuthenticationRequirement::Permission(_), RouteMutation::ReadOnly) => {
                    AUTHORIZED_READ_ROUTE_ERROR_STATUSES
                }
                (AuthenticationRequirement::Permission(_), RouteMutation::Mutating) => {
                    AUTHORIZED_MUTATING_ROUTE_ERROR_STATUSES
                }
            },
        }
    }
}
impl SuccessStatus {
    #[must_use]
    pub fn transport_status(self) -> TransportStatus {
        match self {
            Self::Code200 => TransportStatus::from(KnownHttpStatus::Ok),
            Self::Code201 => TransportStatus::from(KnownHttpStatus::Created),
            Self::Code204 => TransportStatus::from(KnownHttpStatus::NoContent),
        }
    }
}
#[derive(optml::Optml, Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuthenticationRequirement {
    Authenticated,
    Permission(ContractStr),
    Public,
}
#[derive(optml::Optml, Clone, Copy, Debug, PartialEq, Eq)]
pub enum MutationKind {
    ReadOnly,
    Mutating,
}
#[derive(optml::Optml, Clone, Copy, Debug, PartialEq, Eq)]
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
#[derive(optml::Optml, Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConfirmationRequirement {
    NotRequired,
    Required,
}
#[derive(optml::Optml, Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActionContract {
    route: RouteContract,
    confirmation: ConfirmationRequirement,
    operation: OperationKind,
}
#[derive(optml::Optml, Clone, Debug, PartialEq, Eq, newtype::AsRefTarget, newtype::FromInner)]
pub struct ActionContracts(bounded_types::BoundedVec<ActionContract, 0, { usize::MAX }>);
impl From<Vec<ActionContract>> for ActionContracts {
    fn from(value: Vec<ActionContract>) -> Self {
        Self::from(bounded_types::BoundedVec::from_max_iter(value))
    }
}
impl ActionContracts {
    #[must_use]
    pub fn from_max_iter<Values>(values: Values) -> Self
    where
        Values: IntoIterator<Item = ActionContract>,
    {
        Self::from(bounded_types::BoundedVec::from_max_iter(values))
    }
}
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
#[derive(optml::Optml, Clone, Copy, Debug, PartialEq, Eq)]
pub struct RouteContract {
    path: ContractStr,
    authentication: AuthenticationRequirement,
    method: HttpMethod,
    mutation: MutationKind,
    success_status: SuccessStatus,
}
#[derive(optml::Optml, Clone, Debug, PartialEq, Eq, newtype::AsRefTarget, newtype::FromInner)]
pub struct RouteContracts(bounded_types::BoundedVec<RouteContract, 0, { usize::MAX }>);
impl From<Vec<RouteContract>> for RouteContracts {
    fn from(value: Vec<RouteContract>) -> Self {
        Self::from(bounded_types::BoundedVec::from_max_iter(value))
    }
}
impl RouteContracts {
    #[must_use]
    pub fn from_max_iter<Values>(values: Values) -> Self
    where
        Values: IntoIterator<Item = RouteContract>,
    {
        Self::from(bounded_types::BoundedVec::from_max_iter(values))
    }
}
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
            path,
            authentication,
            method,
            mutation,
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
#[derive(optml::Optml, Clone, Debug, PartialEq, Eq)]
pub struct PageContract {
    actions: ActionContracts,
    fields: FieldContracts,
    path: ContractStr,
    routes: RouteContracts,
    title: ContractStr,
}
#[derive(optml::Optml, Clone, Debug, PartialEq, Eq, newtype::AsRefTarget)]
pub struct TransportBody(bounded_types::BoundedVec<u8, 0, FRONTEND_CONTRACT_BODY_MAX_BYTES>);
impl TryFrom<Vec<u8>> for TransportBody {
    type Error = FrontendContractBodyError;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        bounded_types::BoundedVec::try_from(value)
            .map(Self)
            .map_err(FrontendContractBodyError::from)
    }
}
#[derive(optml::Optml, Clone, Debug, PartialEq, Eq)]
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
#[derive(optml::Optml, Clone, Debug, PartialEq, Eq, newtype::AsRefStr, newtype::BoundedString)]
#[bounded_string(max = 255usize, min = 1usize)]
pub struct TransportIdempotencyKey(String);
#[derive(optml::Optml, Clone, Debug, PartialEq, Eq, newtype::AsRefStr, newtype::BoundedString)]
#[bounded_string(max = 20usize, min = 1usize)]
pub struct TransportIfMatch(String);
#[derive(
    optml::Optml, Clone, Debug, Default, PartialEq, Eq, newtype::AsRefStr, newtype::BoundedString,
)]
#[bounded_string(max = 8192usize)]
pub struct TransportPath(String);
#[derive(
    optml::Optml,
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
    error = HttpStatusTryFromU16Error,
    validator = TransportStatus::validate
)]
pub struct TransportStatus(u16);
impl From<KnownHttpStatus> for TransportStatus {
    fn from(value: KnownHttpStatus) -> Self {
        Self(value.get())
    }
}
impl TransportStatus {
    #[allow(clippy::single_call_fn, clippy::trivially_copy_pass_by_ref)] // derive-generated TryFrom owns the single call and borrows the inner value
    fn validate(value: &u16) -> Result<(), HttpStatusTryFromU16Error> {
        if (100u16..1_000u16).contains(value) {
            Ok(())
        } else {
            Err(HttpStatusTryFromU16Error)
        }
    }
}
#[derive(optml::Optml, Clone, Debug, PartialEq, Eq, newtype::AsRefStr, newtype::BoundedString)]
#[bounded_string(max = 128usize, min = 1usize)]
pub struct TransportRetryAfter(String);
#[derive(optml::Optml, Clone, Debug, PartialEq, Eq)]
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
#[derive(
    optml::Optml, Clone, Debug, Default, PartialEq, Eq, newtype::Display, newtype::FromInner,
)]
pub struct TransportError(to_err_string::ErrorText);
impl TryFrom<String> for TransportError {
    type Error = to_err_string::ErrorTextTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        to_err_string::ErrorText::try_from(value).map(Self)
    }
}
pub trait Transport {
    fn send(
        &self,
        request: TransportRequest,
    ) -> impl Future<Output = Result<TransportResponse, TransportError>> + '_;
}
#[derive(optml::Optml, Clone, Debug, PartialEq, Eq)]
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
mod tests;
