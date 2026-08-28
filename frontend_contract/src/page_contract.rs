#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, PartialEq, Eq)]
pub struct PageContract {
    actions: super::super::ActionContracts,
    fields: super::super::FieldContracts,
    path: super::super::ContractStr,
    routes: super::super::RouteContracts,
    title: super::super::ContractStr,
}

impl PageContract {
    #[must_use]
    pub const fn new(
        actions: super::super::ActionContracts,
        fields: super::super::FieldContracts,
        path: super::super::ContractStr,
        routes: super::super::RouteContracts,
        title: super::super::ContractStr,
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
    pub const fn actions(&self) -> &super::super::ActionContracts {
        &self.actions
    }
    #[must_use]
    pub const fn fields(&self) -> &super::super::FieldContracts {
        &self.fields
    }
    #[must_use]
    pub const fn path(&self) -> super::super::ContractStr {
        self.path
    }
    #[must_use]
    pub const fn routes(&self) -> &super::super::RouteContracts {
        &self.routes
    }
    #[must_use]
    pub const fn title(&self) -> super::super::ContractStr {
        self.title
    }
}
