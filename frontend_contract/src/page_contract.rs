#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, PartialEq, Eq)]
pub struct PageContract {
    actions: crate::action_contracts::ActionContracts,
    fields: crate::field_contracts::FieldContracts,
    path: crate::contract_str::ContractStr,
    routes: crate::route_contracts::RouteContracts,
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
    #[must_use]
    pub const fn actions(&self) -> &crate::action_contracts::ActionContracts {
        &self.actions
    }
    #[must_use]
    pub const fn fields(&self) -> &crate::field_contracts::FieldContracts {
        &self.fields
    }
    #[must_use]
    pub const fn path(&self) -> crate::contract_str::ContractStr {
        self.path
    }
    #[must_use]
    pub const fn routes(&self) -> &crate::route_contracts::RouteContracts {
        &self.routes
    }
    #[must_use]
    pub const fn title(&self) -> crate::contract_str::ContractStr {
        self.title
    }
}
