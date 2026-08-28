#![allow(
    clippy::arbitrary_source_item_ordering,
    clippy::module_inception,
    reason = "the flat source facade keeps its owner adjacent to implementation while declaring sibling modules"
)]
#[path = "action_contract.rs"]
mod action_contract;
#[path = "action_contracts.rs"]
mod action_contracts;
#[path = "authentication_requirement.rs"]
mod authentication_requirement;
#[path = "confirmation_requirement.rs"]
mod confirmation_requirement;
#[path = "http_method.rs"]
mod http_method;
#[path = "mutation_kind.rs"]
mod mutation_kind;
#[path = "operation_kind.rs"]
mod operation_kind;
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq)]
pub struct RouteContract {
    path: super::ContractStr,
    authentication: AuthenticationRequirement,
    method: HttpMethod,
    mutation: MutationKind,
    success_status: SuccessStatus,
}

impl RouteContract {
    #[must_use]
    pub const fn new(
        authentication: AuthenticationRequirement,
        method: HttpMethod,
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
    pub const fn method(self) -> HttpMethod {
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
#[path = "route_contracts.rs"]
mod route_contracts;
#[path = "route_error_policy.rs"]
mod route_error_policy;
#[path = "route_error_status.rs"]
mod route_error_status;
#[path = "success_status.rs"]
mod success_status;

pub use action_contract::ActionContract;
pub use action_contracts::ActionContracts;
pub use authentication_requirement::AuthenticationRequirement;
pub use confirmation_requirement::ConfirmationRequirement;
pub use http_method::HttpMethod;
pub use mutation_kind::MutationKind;
pub use operation_kind::OperationKind;
pub use route_contracts::RouteContracts;
pub use route_error_policy::RouteErrorPolicy;
pub use route_error_status::RouteErrorStatus;
pub use success_status::SuccessStatus;

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
                crate::domain_types::RouteMutation::ReadOnly,
            ),
            super::PUBLIC_READ_ROUTE_ERROR_STATUSES,
        );
    }
}
