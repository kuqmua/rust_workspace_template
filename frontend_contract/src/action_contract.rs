#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq,
)]
pub struct ActionContract {
    #[getters(copy)]
    route: crate::route_contract::RouteContract,
    #[getters(copy)]
    confirmation: crate::confirmation_requirement::ConfirmationRequirement,
    #[getters(copy)]
    operation: crate::operation_kind::OperationKind,
}

impl ActionContract {
    #[must_use]
    pub const fn new(
        operation_kind: crate::operation_kind::OperationKind,
        route_contract: crate::route_contract::RouteContract,
    ) -> Self {
        Self {
            confirmation: crate::confirmation_requirement::ConfirmationRequirement::NotRequired,
            operation: operation_kind,
            route: route_contract,
        }
    }

    #[must_use]
    pub const fn with_confirmation(
        mut self,
        confirmation_requirement: crate::confirmation_requirement::ConfirmationRequirement,
    ) -> Self {
        self.confirmation = confirmation_requirement;
        self
    }
}
