#![allow(clippy::arbitrary_source_item_ordering)] // contract implementations keep constructors before accessors and fluent modifiers
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ContractStr(&'static str);
impl From<&'static str> for ContractStr {
    fn from(value: &'static str) -> Self {
        Self(value)
    }
}
impl AsRef<str> for ContractStr {
    fn as_ref(&self) -> &str {
        self.0
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ContractI64(i64);
impl ContractI64 {
    pub const I16_MAX: Self = Self(32_767i64);
    pub const I16_MIN: Self = Self(-32_768i64);
    pub const I32_MAX: Self = Self(2_147_483_647i64);
    pub const I32_MIN: Self = Self(-2_147_483_648i64);
    pub const MAX: Self = Self(i64::MAX);
    pub const MIN: Self = Self(i64::MIN);
}
impl From<i64> for ContractI64 {
    fn from(value: i64) -> Self {
        Self(value)
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
    const TYPE_CONTRACT: TypeContract;
}
#[derive(Clone, Debug, Default, PartialEq, Eq, newtype::BoundedString)]
#[bounded_string(max = 1_048_576usize)]
pub struct FormValue(String);
impl AsRef<str> for FormValue {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FormValueRef<'value_lt>(&'value_lt str);
impl<'value_lt> From<&'value_lt str> for FormValueRef<'value_lt> {
    fn from(value: &'value_lt str) -> Self {
        Self(value)
    }
}
impl AsRef<str> for FormValueRef<'_> {
    fn as_ref(&self) -> &str {
        self.0
    }
}
#[derive(Clone, Debug, Default, PartialEq, Eq, newtype::BoundedString)]
#[bounded_string(max = 65536usize)]
pub struct FormValueEr(String);
impl std::fmt::Display for FormValueEr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
pub trait FormValueContract: Sized {
    fn format_form_value(&self) -> Result<FormValue, FormValueEr>;
    fn parse_form_value(value: FormValueRef<'_>) -> Result<Self, FormValueEr>;
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FormFieldEr {
    error: FormValueEr,
    field: ContractStr,
}
impl FormFieldEr {
    #[must_use]
    pub const fn new(error: FormValueEr, field: ContractStr) -> Self {
        Self { error, field }
    }
    #[must_use]
    pub const fn error(&self) -> &FormValueEr {
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldOrder(usize);
impl From<usize> for FieldOrder {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
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
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldContracts(Vec<FieldContract>);
impl From<Vec<FieldContract>> for FieldContracts {
    fn from(value: Vec<FieldContract>) -> Self {
        Self(value)
    }
}
impl AsRef<[FieldContract]> for FieldContracts {
    fn as_ref(&self) -> &[FieldContract] {
        self.0.as_slice()
    }
}
impl FieldContract {
    #[must_use]
    pub const fn new(name: ContractStr, label: ContractStr, type_contract: TypeContract) -> Self {
        Self {
            creatable: FieldCapability::Disabled,
            filterable: FieldCapability::Disabled,
            label,
            name,
            order: FieldOrder(0usize),
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
    Delete,
    Get,
    Patch,
    Post,
    Put,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SuccessStatus {
    Code200,
    Code201,
    Code204,
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
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionContracts(Vec<ActionContract>);
impl From<Vec<ActionContract>> for ActionContracts {
    fn from(value: Vec<ActionContract>) -> Self {
        Self(value)
    }
}
impl AsRef<[ActionContract]> for ActionContracts {
    fn as_ref(&self) -> &[ActionContract] {
        self.0.as_slice()
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RouteContract {
    authentication: AuthenticationRequirement,
    method: HttpMethod,
    mutation: MutationKind,
    path: ContractStr,
    success_status: SuccessStatus,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RouteContracts(Vec<RouteContract>);
impl From<Vec<RouteContract>> for RouteContracts {
    fn from(value: Vec<RouteContract>) -> Self {
        Self(value)
    }
}
impl AsRef<[RouteContract]> for RouteContracts {
    fn as_ref(&self) -> &[RouteContract] {
        self.0.as_slice()
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
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransportBody(Vec<u8>);
impl From<Vec<u8>> for TransportBody {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}
impl AsRef<[u8]> for TransportBody {
    fn as_ref(&self) -> &[u8] {
        self.0.as_slice()
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransportRequest {
    body: TransportBody,
    path: TransportPath,
    route: RouteContract,
}
impl TransportRequest {
    #[must_use]
    pub const fn new(body: TransportBody, path: TransportPath, route: RouteContract) -> Self {
        Self { body, path, route }
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
}
#[derive(Clone, Debug, Default, PartialEq, Eq, newtype::BoundedString)]
#[bounded_string(max = 8192usize)]
pub struct TransportPath(String);
impl AsRef<str> for TransportPath {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TransportStatus(u16);
impl From<u16> for TransportStatus {
    fn from(value: u16) -> Self {
        Self(value)
    }
}
impl From<TransportStatus> for u16 {
    fn from(value: TransportStatus) -> Self {
        value.0
    }
}
impl std::fmt::Display for TransportStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransportResponse {
    body: TransportBody,
    status: TransportStatus,
}
impl TransportResponse {
    #[must_use]
    pub const fn new(body: TransportBody, status: TransportStatus) -> Self {
        Self { body, status }
    }
    #[must_use]
    pub const fn body(&self) -> &TransportBody {
        &self.body
    }
    #[must_use]
    pub const fn status(&self) -> TransportStatus {
        self.status
    }
}
#[derive(Clone, Debug, Default, PartialEq, Eq, newtype::BoundedString)]
#[bounded_string(max = 65536usize)]
pub struct TransportEr(String);
impl std::fmt::Display for TransportEr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
pub trait Transport {
    fn send(
        &self,
        request: TransportRequest,
    ) -> std::pin::Pin<Box<dyn Future<Output = Result<TransportResponse, TransportEr>> + '_>>;
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ClientEr {
    Decode(FormValueEr),
    Encode(FormValueEr),
    Status {
        actual: TransportStatus,
        expected: TransportStatus,
    },
    Transport(TransportEr),
    UnexpectedResponse,
}
impl std::fmt::Display for ClientEr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Decode(value) => write!(f, "failed to decode response: {value}"),
            Self::Encode(value) => write!(f, "failed to encode request: {value}"),
            Self::Status { actual, expected } => {
                write!(f, "expected HTTP {expected}, received HTTP {actual}")
            }
            Self::Transport(value) => write!(f, "transport failed: {value}"),
            Self::UnexpectedResponse => f.write_str("server returned an error response"),
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
    fn contracts_preserve_typed_metadata() {
        let type_contract = super::TypeContract::new(
            super::InputKind::Number,
            super::ValueFormat::Int64,
            super::Nullability::NonNullable,
        )
        .with_minimum(super::NumericBound::Inclusive(super::ContractI64::from(1)))
        .with_step(super::InputStep::Integer);
        let field = super::FieldContract::new(
            super::ContractStr::from("id"),
            super::ContractStr::from("ID"),
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
            super::AuthenticationRequirement::Permission(super::ContractStr::from("users:update")),
            super::HttpMethod::Patch,
            super::MutationKind::Mutating,
            super::ContractStr::from("/users/{id}"),
            super::SuccessStatus::Code204,
        );
        assert_eq!(route.mutation(), super::MutationKind::Mutating);
        assert_eq!(route.method(), super::HttpMethod::Patch);
        assert_eq!(route.path().as_ref(), "/users/{id}");
    }
}
