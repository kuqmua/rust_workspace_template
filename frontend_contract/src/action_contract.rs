#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActionContract {
    route: super::RouteContract,
    confirmation: super::ConfirmationRequirement,
    operation: super::OperationKind,
}

impl ActionContract {
    #[must_use]
    pub const fn new(operation: super::OperationKind, route: super::RouteContract) -> Self {
        Self {
            confirmation: super::ConfirmationRequirement::NotRequired,
            operation,
            route,
        }
    }
    #[must_use]
    pub const fn confirmation(self) -> super::ConfirmationRequirement {
        self.confirmation
    }
    #[must_use]
    pub const fn operation(self) -> super::OperationKind {
        self.operation
    }
    #[must_use]
    pub const fn route(self) -> super::RouteContract {
        self.route
    }
    #[must_use]
    pub const fn with_confirmation(mut self, value: super::ConfirmationRequirement) -> Self {
        self.confirmation = value;
        self
    }
}
