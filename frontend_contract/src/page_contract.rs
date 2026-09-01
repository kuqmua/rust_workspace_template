#[derive(generate_accessor::Getters)]
#[getters(bare)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, PartialEq, Eq)]
pub struct PageContract {
    actions: crate::action_contracts::ActionContracts,
    fields: crate::field_contracts::FieldContracts,
    #[getters(copy)]
    path: crate::contract_str::ContractStr,
    routes: crate::route_contracts::RouteContracts,
    #[getters(copy)]
    title: crate::contract_str::ContractStr,
}

impl PageContract {
    #[must_use]
    pub const fn new(
        actions: crate::action_contracts::ActionContracts,
        fields: crate::field_contracts::FieldContracts,
        path: crate::contract_str::ContractStr,
        routes: crate::route_contracts::RouteContracts,
        title: crate::contract_str::ContractStr,
    ) -> Self {
        Self {
            actions,
            fields,
            path,
            routes,
            title,
        }
    }
}
