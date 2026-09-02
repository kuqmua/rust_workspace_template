#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "the flat source facade keeps its owner adjacent to implementation while declaring sibling modules"
)]
#[cfg(test)]
mod tests {
    #[test]
    fn test_type_contract_preserves_input_metadata() {
        let contract = crate::type_contract::TypeContract::new(
            crate::input_kind::InputKind::Number,
            crate::value_format::ValueFormat::Int64,
            crate::nullability::Nullability::NonNullable,
        );
        assert_eq!(contract.input_kind(), crate::input_kind::InputKind::Number);
        assert_eq!(contract.format(), crate::value_format::ValueFormat::Int64);
    }
}
#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq,
)]
pub struct FieldContract {
    #[getters(skip)]
    filters: crate::filter_contracts::FilterContracts,
    #[getters(copy)]
    label: crate::contract_str::ContractStr,
    #[getters(copy)]
    name: crate::contract_str::ContractStr,
    #[getters(copy)]
    placeholder: crate::field_placeholder::FieldPlaceholder,
    #[getters(copy)]
    type_contract: crate::type_contract::TypeContract,
    #[getters(copy)]
    order: crate::field_order::FieldOrder,
    #[getters(copy)]
    creatable: crate::field_capability::FieldCapability,
    #[getters(copy)]
    filterable: crate::field_capability::FieldCapability,
    #[getters(copy)]
    primary_key: crate::primary_key_kind::PrimaryKeyKind,
    #[getters(copy)]
    readable: crate::field_capability::FieldCapability,
    #[getters(copy)]
    sortable: crate::field_capability::FieldCapability,
    #[getters(copy)]
    updatable: crate::field_capability::FieldCapability,
    #[getters(copy)]
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
    pub fn filters(&self) -> &[crate::filter_operation::FilterOperation] {
        self.filters.as_ref()
    }

    #[must_use]
    pub const fn with_creatable(
        mut self,
        field_capability: crate::field_capability::FieldCapability,
    ) -> Self {
        self.creatable = field_capability;
        self
    }
    #[must_use]
    pub const fn with_filterable(
        mut self,
        field_capability: crate::field_capability::FieldCapability,
    ) -> Self {
        self.filterable = field_capability;
        self
    }
    #[must_use]
    pub const fn with_filters(
        mut self,
        filter_contracts: crate::filter_contracts::FilterContracts,
    ) -> Self {
        self.filters = filter_contracts;
        self
    }
    #[must_use]
    pub const fn with_order(mut self, field_order: crate::field_order::FieldOrder) -> Self {
        self.order = field_order;
        self
    }
    #[must_use]
    pub const fn with_placeholder(
        mut self,
        field_placeholder: crate::field_placeholder::FieldPlaceholder,
    ) -> Self {
        self.placeholder = field_placeholder;
        self
    }
    #[must_use]
    pub const fn with_primary_key(
        mut self,
        primary_key_kind: crate::primary_key_kind::PrimaryKeyKind,
    ) -> Self {
        self.primary_key = primary_key_kind;
        self
    }
    #[must_use]
    pub const fn with_readable(
        mut self,
        field_capability: crate::field_capability::FieldCapability,
    ) -> Self {
        self.readable = field_capability;
        self
    }
    #[must_use]
    pub const fn with_sortable(
        mut self,
        field_capability: crate::field_capability::FieldCapability,
    ) -> Self {
        self.sortable = field_capability;
        self
    }
    #[must_use]
    pub const fn with_updatable(
        mut self,
        field_capability: crate::field_capability::FieldCapability,
    ) -> Self {
        self.updatable = field_capability;
        self
    }
    #[must_use]
    pub const fn with_visibility(
        mut self,
        field_visibility: crate::field_visibility::FieldVisibility,
    ) -> Self {
        self.visibility = field_visibility;
        self
    }
}
