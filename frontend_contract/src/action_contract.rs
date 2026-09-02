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
    pub const fn with_confirmation(
        mut self,
        value: crate::confirmation_requirement::ConfirmationRequirement,
    ) -> Self {
        self.confirmation = value;
        self
    }
}
