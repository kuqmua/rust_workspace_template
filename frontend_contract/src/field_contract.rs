#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "the flat source facade keeps its owner adjacent to implementation while declaring sibling modules"
)]
#[cfg(test)]
mod tests {
    #[test]
    fn type_contract_preserves_input_metadata() {
        let contract = crate::type_contract::TypeContract::new(
            crate::input_kind::InputKind::Number,
            crate::value_format::ValueFormat::Int64,
            crate::nullability::Nullability::NonNullable,
        );
        assert_eq!(contract.input_kind(), crate::input_kind::InputKind::Number);
        assert_eq!(contract.format(), crate::value_format::ValueFormat::Int64);
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldContract {
    filters: crate::filter_contracts::FilterContracts,
    label: crate::contract_str::ContractStr,
    name: crate::contract_str::ContractStr,
    placeholder: crate::field_placeholder::FieldPlaceholder,
    type_contract: crate::type_contract::TypeContract,
    order: crate::field_order::FieldOrder,
    creatable: crate::field_capability::FieldCapability,
    filterable: crate::field_capability::FieldCapability,
    primary_key: crate::primary_key_kind::PrimaryKeyKind,
    readable: crate::field_capability::FieldCapability,
    sortable: crate::field_capability::FieldCapability,
    updatable: crate::field_capability::FieldCapability,
    visibility: crate::field_visibility::FieldVisibility,
}
impl FieldContract {
    #[must_use]
    pub fn new(
        name: crate::contract_str::ContractStr,
        label: crate::contract_str::ContractStr,
        type_contract: crate::type_contract::TypeContract,
    ) -> Self {
        Self {
            creatable: crate::field_capability::FieldCapability::Disabled,
            filterable: crate::field_capability::FieldCapability::Disabled,
            filters: crate::filter_contracts::FilterContracts::from(
                crate::empty_filter_contracts::EMPTY_FILTER_CONTRACTS,
            ),
            label,
            name,
            order: crate::field_order::FieldOrder::from(constants_usize::ZERO),
            placeholder: crate::field_placeholder::FieldPlaceholder::None,
            primary_key: crate::primary_key_kind::PrimaryKeyKind::NonPrimary,
            readable: crate::field_capability::FieldCapability::Disabled,
            sortable: crate::field_capability::FieldCapability::Disabled,
            type_contract,
            updatable: crate::field_capability::FieldCapability::Disabled,
            visibility: crate::field_visibility::FieldVisibility::Visible,
        }
    }
    #[must_use]
    pub const fn creatable(self) -> crate::field_capability::FieldCapability {
        self.creatable
    }
    #[must_use]
    pub const fn label(self) -> crate::contract_str::ContractStr {
        self.label
    }
    #[must_use]
    pub const fn filterable(self) -> crate::field_capability::FieldCapability {
        self.filterable
    }
    #[must_use]
    pub fn filters(&self) -> &[crate::filter_operation::FilterOperation] {
        self.filters.as_ref()
    }
    #[must_use]
    pub const fn name(self) -> crate::contract_str::ContractStr {
        self.name
    }
    #[must_use]
    pub const fn order(self) -> crate::field_order::FieldOrder {
        self.order
    }
    #[must_use]
    pub const fn placeholder(self) -> crate::field_placeholder::FieldPlaceholder {
        self.placeholder
    }
    #[must_use]
    pub const fn primary_key(self) -> crate::primary_key_kind::PrimaryKeyKind {
        self.primary_key
    }
    #[must_use]
    pub const fn readable(self) -> crate::field_capability::FieldCapability {
        self.readable
    }
    #[must_use]
    pub const fn sortable(self) -> crate::field_capability::FieldCapability {
        self.sortable
    }
    #[must_use]
    pub const fn type_contract(self) -> crate::type_contract::TypeContract {
        self.type_contract
    }
    #[must_use]
    pub const fn updatable(self) -> crate::field_capability::FieldCapability {
        self.updatable
    }
    #[must_use]
    pub const fn visibility(self) -> crate::field_visibility::FieldVisibility {
        self.visibility
    }
    #[must_use]
    pub const fn with_creatable(mut self, value: crate::field_capability::FieldCapability) -> Self {
        self.creatable = value;
        self
    }
    #[must_use]
    pub const fn with_filterable(
        mut self,
        value: crate::field_capability::FieldCapability,
    ) -> Self {
        self.filterable = value;
        self
    }
    #[must_use]
    pub const fn with_filters(mut self, value: crate::filter_contracts::FilterContracts) -> Self {
        self.filters = value;
        self
    }
    #[must_use]
    pub const fn with_order(mut self, value: crate::field_order::FieldOrder) -> Self {
        self.order = value;
        self
    }
    #[must_use]
    pub const fn with_placeholder(
        mut self,
        value: crate::field_placeholder::FieldPlaceholder,
    ) -> Self {
        self.placeholder = value;
        self
    }
    #[must_use]
    pub const fn with_primary_key(
        mut self,
        value: crate::primary_key_kind::PrimaryKeyKind,
    ) -> Self {
        self.primary_key = value;
        self
    }
    #[must_use]
    pub const fn with_readable(mut self, value: crate::field_capability::FieldCapability) -> Self {
        self.readable = value;
        self
    }
    #[must_use]
    pub const fn with_sortable(mut self, value: crate::field_capability::FieldCapability) -> Self {
        self.sortable = value;
        self
    }
    #[must_use]
    pub const fn with_updatable(mut self, value: crate::field_capability::FieldCapability) -> Self {
        self.updatable = value;
        self
    }
    #[must_use]
    pub const fn with_visibility(
        mut self,
        value: crate::field_visibility::FieldVisibility,
    ) -> Self {
        self.visibility = value;
        self
    }
}
