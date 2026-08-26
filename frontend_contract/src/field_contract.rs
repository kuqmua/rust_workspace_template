#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
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
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputKind {
    Checkbox,
    Date,
    DateTime,
    Number,
    Text,
    Time,
    Uuid,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq)]
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
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nullability {
    NonNullable,
    Nullable,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq)]
pub enum CapabilitySupport {
    Supported,
    Unsupported,
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
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    newtype::AsRefInner,
    newtype::FromInner,
)]
pub struct FilterContracts(&'static [FilterOperation]);
pub trait HasFilterContracts {
    const FILTER_CONTRACTS: &'static [FilterOperation];
    #[must_use]
    fn filter_contracts() -> FilterContracts {
        FilterContracts::from(Self::FILTER_CONTRACTS)
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputStep {
    Any,
    Decimal,
    Integer,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq)]
pub enum NumericBound {
    None,
    Inclusive(ContractI64),
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    newtype::FromInner,
)]
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
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq)]
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
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq)]
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
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    newtype::AsRefStr,
    newtype::BoundedString,
)]
#[bounded_string(max = constants_usize::VALUE_1_048_576)]
pub struct FormValue(String);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    newtype::AsRefInner,
    newtype::FromInner,
)]
pub struct FormValueRef<'value_lt>(&'value_lt str);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    newtype::AsRefStr,
    newtype::FromInner,
)]
pub struct FormFieldNameRef<'field_lt>(&'field_lt str);
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
pub struct FormValueError(to_err_string::domain_types::ErrorText);
impl TryFrom<String> for FormValueError {
    type Error = to_err_string::domain_types::ErrorTextTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        to_err_string::domain_types::ErrorText::try_from(value).map(Self)
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
#[bounded_string(max = constants_usize::VALUE_1_048_576)]
pub struct FilterWireJson(String);
pub trait FormValueContract: Sized {
    fn format_form_value(&self) -> Result<FormValue, FormValueError>;
    fn parse_form_value(value: FormValueRef<'_>) -> Result<Self, FormValueError>;
}
pub trait FilterFormValueContract {
    fn parse_filter_form_value(value: FormValueRef<'_>) -> Result<FilterWireJson, FormValueError>;
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, PartialEq, Eq)]
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
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldCapability {
    Disabled,
    Enabled,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrimaryKeyKind {
    NonPrimary,
    Primary,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    newtype::FromInner,
)]
pub struct FieldOrder(usize);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldVisibility {
    Hidden,
    Visible,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldPlaceholder {
    None,
    Value(ContractStr),
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq)]
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
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    PartialEq,
    Eq,
    newtype::AsRefTarget,
    newtype::FromInner,
)]
pub struct FieldContracts(
    bounded_types::domain_types::vector::BoundedVec<FieldContract, 0, { usize::MAX }>,
);
impl TryFrom<Vec<FieldContract>> for FieldContracts {
    type Error = bounded_types::domain_types::BoundedValueError;
    fn try_from(value: Vec<FieldContract>) -> Result<Self, Self::Error> {
        bounded_types::domain_types::vector::BoundedVec::try_from_collection_vec(value)
            .map(Self::from)
    }
}
impl FieldContracts {
    #[must_use]
    pub fn from_max_iter<Values>(values: Values) -> Self
    where
        Values: IntoIterator<Item = FieldContract>,
    {
        Self::from(bounded_types::domain_types::vector::BoundedVec::from_max_iter(values))
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
            order: FieldOrder::from(constants_usize::ZERO),
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

#[cfg(test)]
mod tests {
    #[test]
    fn type_contract_preserves_input_metadata() {
        let contract = super::TypeContract::new(
            super::InputKind::Number,
            super::ValueFormat::Int64,
            super::Nullability::NonNullable,
        );
        assert_eq!(contract.input_kind(), super::InputKind::Number);
        assert_eq!(contract.format(), super::ValueFormat::Int64);
    }
}
