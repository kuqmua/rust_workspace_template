#![allow(
    clippy::arbitrary_source_item_ordering,
    clippy::module_inception,
    reason = "the flat source facade keeps its owner adjacent to implementation while declaring sibling modules"
)]
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
#[path = "contract_str.rs"]
mod contract_str;
pub use contract_str::*;
#[path = "input_kind.rs"]
mod input_kind;
pub use input_kind::*;
#[path = "value_format.rs"]
mod value_format;
pub use value_format::*;
#[path = "nullability.rs"]
mod nullability;
pub use nullability::*;
#[path = "capability_support.rs"]
mod capability_support;
pub use capability_support::*;
#[path = "filter_operation.rs"]
mod filter_operation;
pub use filter_operation::*;
#[path = "filter_value_shape.rs"]
mod filter_value_shape;
pub use filter_value_shape::*;
#[path = "filter_contracts.rs"]
mod filter_contracts;
pub use filter_contracts::*;
#[path = "has_filter_contracts.rs"]
mod has_filter_contracts;
pub use has_filter_contracts::*;
#[path = "input_step.rs"]
mod input_step;
pub use input_step::*;
#[path = "numeric_bound.rs"]
mod numeric_bound;
pub use numeric_bound::*;
#[path = "contract_i64.rs"]
mod contract_i64;
pub use contract_i64::*;
#[path = "value_example.rs"]
mod value_example;
pub use value_example::*;
#[path = "type_contract.rs"]
mod type_contract;
pub use type_contract::*;
#[path = "has_type_contract.rs"]
mod has_type_contract;
pub use has_type_contract::*;
#[path = "form_value.rs"]
mod form_value;
pub use form_value::*;
#[path = "form_value_ref.rs"]
mod form_value_ref;
pub use form_value_ref::*;
#[path = "form_field_name_ref.rs"]
mod form_field_name_ref;
pub use form_field_name_ref::*;
#[path = "form_value_error.rs"]
mod form_value_error;
pub use form_value_error::*;
#[path = "filter_wire_json.rs"]
mod filter_wire_json;
pub use filter_wire_json::*;
#[path = "form_value_contract.rs"]
mod form_value_contract;
pub use form_value_contract::*;
#[path = "filter_form_value_contract.rs"]
mod filter_form_value_contract;
pub use filter_form_value_contract::*;
#[path = "form_field_error.rs"]
mod form_field_error;
pub use form_field_error::*;
#[path = "field_capability.rs"]
mod field_capability;
pub use field_capability::*;
#[path = "primary_key_kind.rs"]
mod primary_key_kind;
pub use primary_key_kind::*;
#[path = "field_order.rs"]
mod field_order;
pub use field_order::*;
#[path = "field_visibility.rs"]
mod field_visibility;
pub use field_visibility::*;
#[path = "field_placeholder.rs"]
mod field_placeholder;
pub use field_placeholder::*;
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
#[path = "field_contracts.rs"]
mod field_contracts;
pub use field_contracts::*;
#[path = "empty_filter_contracts.rs"]
mod empty_filter_contracts;
use empty_filter_contracts::EMPTY_FILTER_CONTRACTS;
