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
pub use crate::capability_support::*;
pub use crate::contract_i64::*;
pub use crate::contract_str::*;
pub use crate::field_capability::*;
pub use crate::field_order::*;
pub use crate::field_placeholder::*;
pub use crate::field_visibility::*;
pub use crate::filter_contracts::*;
pub use crate::filter_form_value_contract::*;
pub use crate::filter_operation::*;
pub use crate::filter_value_shape::*;
pub use crate::filter_wire_json::*;
pub use crate::form_field_error::*;
pub use crate::form_field_name_ref::*;
pub use crate::form_value::*;
pub use crate::form_value_contract::*;
pub use crate::form_value_error::*;
pub use crate::form_value_ref::*;
pub use crate::has_filter_contracts::*;
pub use crate::has_type_contract::*;
pub use crate::input_kind::*;
pub use crate::input_step::*;
pub use crate::nullability::*;
pub use crate::numeric_bound::*;
pub use crate::primary_key_kind::*;
pub use crate::type_contract::*;
pub use crate::value_example::*;
pub use crate::value_format::*;
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
use crate::empty_filter_contracts::EMPTY_FILTER_CONTRACTS;
pub use crate::field_contracts::*;
