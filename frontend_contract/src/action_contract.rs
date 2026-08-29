#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActionContract {
    route: crate::route_contract::RouteContract,
    confirmation: crate::confirmation_requirement::ConfirmationRequirement,
    operation: crate::operation_kind::OperationKind,
}

impl ActionContract {
    #[must_use]
    pub const fn new(
        operation: crate::operation_kind::OperationKind,
        route: crate::route_contract::RouteContract,
    ) -> Self {
        Self {
            confirmation: crate::confirmation_requirement::ConfirmationRequirement::NotRequired,
            operation,
            route,
        }
    }
    #[must_use]
    pub const fn confirmation(self) -> crate::confirmation_requirement::ConfirmationRequirement {
        self.confirmation
    }
    #[must_use]
    pub const fn operation(self) -> crate::operation_kind::OperationKind {
        self.operation
    }
    #[must_use]
    pub const fn route(self) -> crate::route_contract::RouteContract {
        self.route
    }
    #[must_use]
    pub const fn with_confirmation(
        mut self,
        value: crate::confirmation_requirement::ConfirmationRequirement,
    ) -> Self {
        self.confirmation = value;
        self
    }
}
