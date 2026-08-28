#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "the flat source facade keeps its owner adjacent to implementation while declaring sibling modules"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq)]
pub struct RouteContract {
    path: super::ContractStr,
    authentication: AuthenticationRequirement,
    method: super::RouteMethod,
    mutation: MutationKind,
    success_status: SuccessStatus,
}

impl RouteContract {
    #[must_use]
    pub const fn new(
        authentication: AuthenticationRequirement,
        method: super::RouteMethod,
        mutation: MutationKind,
        path: super::ContractStr,
        success_status: SuccessStatus,
    ) -> Self {
        Self {
            path,
            authentication,
            method,
            mutation,
            success_status,
        }
    }
    #[must_use]
    pub const fn authentication(self) -> AuthenticationRequirement {
        self.authentication
    }
    #[must_use]
    pub const fn method(self) -> super::RouteMethod {
        self.method
    }
    #[must_use]
    pub const fn mutation(self) -> MutationKind {
        self.mutation
    }
    #[must_use]
    pub const fn path(self) -> super::ContractStr {
        self.path
    }
    #[must_use]
    pub const fn success_status(self) -> SuccessStatus {
        self.success_status
    }
}

pub use crate::action_contract::ActionContract;
pub use crate::action_contracts::ActionContracts;
pub use crate::authentication_requirement::AuthenticationRequirement;
pub use crate::confirmation_requirement::ConfirmationRequirement;
pub use crate::mutation_kind::MutationKind;
pub use crate::operation_kind::OperationKind;
pub use crate::route_contracts::RouteContracts;
pub use crate::route_error_policy::RouteErrorPolicy;
pub use crate::route_error_status::RouteErrorStatus;
pub use crate::success_status::SuccessStatus;

pub const PUBLIC_AUTH_ROUTE_ERROR_STATUSES: &[RouteErrorStatus] = &[
    RouteErrorStatus::Authentication,
    RouteErrorStatus::PayloadTooLarge,
    RouteErrorStatus::RateLimited,
    RouteErrorStatus::Internal,
];
pub const PUBLIC_READ_ROUTE_ERROR_STATUSES: &[RouteErrorStatus] = &[
    RouteErrorStatus::Internal,
    RouteErrorStatus::PayloadTooLarge,
    RouteErrorStatus::RateLimited,
];
pub const PUBLIC_MUTATING_ROUTE_ERROR_STATUSES: &[RouteErrorStatus] = &[
    RouteErrorStatus::PayloadTooLarge,
    RouteErrorStatus::Validation,
    RouteErrorStatus::RateLimited,
    RouteErrorStatus::Internal,
];
pub const PUBLIC_REFRESH_ROUTE_ERROR_STATUSES: &[RouteErrorStatus] = &[
    RouteErrorStatus::Authentication,
    RouteErrorStatus::PayloadTooLarge,
    RouteErrorStatus::RateLimited,
    RouteErrorStatus::Internal,
];
pub const AUTHENTICATED_READ_ROUTE_ERROR_STATUSES: &[RouteErrorStatus] = &[
    RouteErrorStatus::Authentication,
    RouteErrorStatus::PayloadTooLarge,
    RouteErrorStatus::RateLimited,
    RouteErrorStatus::Internal,
];
pub const AUTHORIZED_READ_ROUTE_ERROR_STATUSES: &[RouteErrorStatus] = &[
    RouteErrorStatus::Authentication,
    RouteErrorStatus::Authorization,
    RouteErrorStatus::PayloadTooLarge,
    RouteErrorStatus::RateLimited,
    RouteErrorStatus::Internal,
];
pub const AUTHORIZED_VALIDATED_READ_ROUTE_ERROR_STATUSES: &[RouteErrorStatus] = &[
    RouteErrorStatus::Authentication,
    RouteErrorStatus::Authorization,
    RouteErrorStatus::PayloadTooLarge,
    RouteErrorStatus::Validation,
    RouteErrorStatus::RateLimited,
    RouteErrorStatus::Internal,
];
pub const AUTHENTICATED_MUTATING_ROUTE_ERROR_STATUSES: &[RouteErrorStatus] = &[
    RouteErrorStatus::Authentication,
    RouteErrorStatus::Authorization,
    RouteErrorStatus::PayloadTooLarge,
    RouteErrorStatus::RateLimited,
    RouteErrorStatus::Internal,
];
pub const AUTHORIZED_MUTATING_ROUTE_ERROR_STATUSES: &[RouteErrorStatus] = &[
    RouteErrorStatus::Authentication,
    RouteErrorStatus::Authorization,
    RouteErrorStatus::Conflict,
    RouteErrorStatus::PayloadTooLarge,
    RouteErrorStatus::Validation,
    RouteErrorStatus::RateLimited,
    RouteErrorStatus::Internal,
];
pub const AUTHORIZED_DELETE_ROUTE_ERROR_STATUSES: &[RouteErrorStatus] = &[
    RouteErrorStatus::Authentication,
    RouteErrorStatus::Authorization,
    RouteErrorStatus::Conflict,
    RouteErrorStatus::PayloadTooLarge,
    RouteErrorStatus::RateLimited,
    RouteErrorStatus::Internal,
];

#[cfg(test)]
mod tests {
    #[test]
    fn public_read_policy_has_stable_statuses() {
        assert_eq!(
            super::RouteErrorPolicy::Default.statuses(
                super::AuthenticationRequirement::Public,
                crate::RouteMutation::ReadOnly,
            ),
            super::PUBLIC_READ_ROUTE_ERROR_STATUSES,
        );
    }
}
