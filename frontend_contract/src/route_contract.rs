#![allow(
    clippy::module_inception,
    reason = "same-named type and function owners require nested modules under the facade"
)]
#[path = "route_contract/action_contract.rs"]
mod action_contract;
#[path = "route_contract/action_contracts.rs"]
mod action_contracts;
#[path = "route_contract/authentication_requirement.rs"]
mod authentication_requirement;
#[path = "route_contract/confirmation_requirement.rs"]
mod confirmation_requirement;
#[path = "route_contract/http_method.rs"]
mod http_method;
#[path = "route_contract/mutation_kind.rs"]
mod mutation_kind;
#[path = "route_contract/operation_kind.rs"]
mod operation_kind;
#[path = "route_contract/route_contract.rs"]
mod route_contract;
#[path = "route_contract/route_contracts.rs"]
mod route_contracts;
#[path = "route_contract/route_error_policy.rs"]
mod route_error_policy;
#[path = "route_contract/route_error_status.rs"]
mod route_error_status;
#[path = "route_contract/success_status.rs"]
mod success_status;

pub use action_contract::ActionContract;
pub use action_contracts::ActionContracts;
pub use authentication_requirement::AuthenticationRequirement;
pub use confirmation_requirement::ConfirmationRequirement;
pub use http_method::HttpMethod;
pub use mutation_kind::MutationKind;
pub use operation_kind::OperationKind;
pub use route_contract::RouteContract;
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
