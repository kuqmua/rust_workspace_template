#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, PartialEq, Eq)]
pub struct PageContract {
    actions: crate::ActionContracts,
    fields: crate::FieldContracts,
    path: crate::ContractStr,
    routes: crate::RouteContracts,
    title: crate::ContractStr,
}

impl PageContract {
    #[must_use]
    pub const fn new(
        actions: crate::ActionContracts,
        fields: crate::FieldContracts,
        path: crate::ContractStr,
        routes: crate::RouteContracts,
        title: crate::ContractStr,
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
    pub const fn actions(&self) -> &crate::ActionContracts {
        &self.actions
    }
    #[must_use]
    pub const fn fields(&self) -> &crate::FieldContracts {
        &self.fields
    }
    #[must_use]
    pub const fn path(&self) -> crate::ContractStr {
        self.path
    }
    #[must_use]
    pub const fn routes(&self) -> &crate::RouteContracts {
        &self.routes
    }
    #[must_use]
    pub const fn title(&self) -> crate::ContractStr {
        self.title
    }
}
