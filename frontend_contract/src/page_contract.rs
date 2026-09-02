#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, PartialEq, Eq)]
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
        action_contracts: crate::action_contracts::ActionContracts,
        field_contracts: crate::field_contracts::FieldContracts,
        path: crate::contract_str::ContractStr,
        route_contracts: crate::route_contracts::RouteContracts,
        title: crate::contract_str::ContractStr,
    ) -> Self {
        Self {
            actions: action_contracts,
            fields: field_contracts,
            path,
            routes: route_contracts,
            title,
        }
    }
}
