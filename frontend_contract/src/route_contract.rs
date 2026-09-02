#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "the flat source facade keeps its owner adjacent to implementation while declaring sibling modules"
)]
#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq,
)]
pub struct RouteContract {
    #[getters(copy)]
    path: crate::contract_str::ContractStr,
    #[getters(copy)]
    authentication: crate::authentication_requirement::AuthenticationRequirement,
    #[getters(copy)]
    method: crate::route_method::RouteMethod,
    #[getters(copy)]
    mutation: crate::mutation_kind::MutationKind,
    #[getters(copy)]
    success_status: crate::success_status::SuccessStatus,
}

impl RouteContract {
    #[must_use]
    pub const fn new(
        authentication: crate::authentication_requirement::AuthenticationRequirement,
        method: crate::route_method::RouteMethod,
        mutation: crate::mutation_kind::MutationKind,
        path: crate::contract_str::ContractStr,
        success_status: crate::success_status::SuccessStatus,
    ) -> Self {
        Self {
            path,
            authentication,
            method,
            mutation,
            success_status,
        }
    }
}
pub const PUBLIC_AUTH_ROUTE_ERROR_STATUSES: &[crate::route_error_status::RouteErrorStatus] = &[
    crate::route_error_status::RouteErrorStatus::Authentication,
    crate::route_error_status::RouteErrorStatus::PayloadTooLarge,
    crate::route_error_status::RouteErrorStatus::RateLimited,
    crate::route_error_status::RouteErrorStatus::Internal,
];
pub const PUBLIC_READ_ROUTE_ERROR_STATUSES: &[crate::route_error_status::RouteErrorStatus] = &[
    crate::route_error_status::RouteErrorStatus::Internal,
    crate::route_error_status::RouteErrorStatus::PayloadTooLarge,
    crate::route_error_status::RouteErrorStatus::RateLimited,
];
pub const PUBLIC_MUTATING_ROUTE_ERROR_STATUSES: &[crate::route_error_status::RouteErrorStatus] = &[
    crate::route_error_status::RouteErrorStatus::PayloadTooLarge,
    crate::route_error_status::RouteErrorStatus::Validation,
    crate::route_error_status::RouteErrorStatus::RateLimited,
    crate::route_error_status::RouteErrorStatus::Internal,
];
pub const PUBLIC_REFRESH_ROUTE_ERROR_STATUSES: &[crate::route_error_status::RouteErrorStatus] = &[
    crate::route_error_status::RouteErrorStatus::Authentication,
    crate::route_error_status::RouteErrorStatus::PayloadTooLarge,
    crate::route_error_status::RouteErrorStatus::RateLimited,
    crate::route_error_status::RouteErrorStatus::Internal,
];
pub const AUTHENTICATED_READ_ROUTE_ERROR_STATUSES:
    &[crate::route_error_status::RouteErrorStatus] = &[
    crate::route_error_status::RouteErrorStatus::Authentication,
    crate::route_error_status::RouteErrorStatus::PayloadTooLarge,
    crate::route_error_status::RouteErrorStatus::RateLimited,
    crate::route_error_status::RouteErrorStatus::Internal,
];
pub const AUTHORIZED_READ_ROUTE_ERROR_STATUSES: &[crate::route_error_status::RouteErrorStatus] = &[
    crate::route_error_status::RouteErrorStatus::Authentication,
    crate::route_error_status::RouteErrorStatus::Authorization,
    crate::route_error_status::RouteErrorStatus::PayloadTooLarge,
    crate::route_error_status::RouteErrorStatus::RateLimited,
    crate::route_error_status::RouteErrorStatus::Internal,
];
pub const AUTHORIZED_VALIDATED_READ_ROUTE_ERROR_STATUSES:
    &[crate::route_error_status::RouteErrorStatus] = &[
    crate::route_error_status::RouteErrorStatus::Authentication,
    crate::route_error_status::RouteErrorStatus::Authorization,
    crate::route_error_status::RouteErrorStatus::PayloadTooLarge,
    crate::route_error_status::RouteErrorStatus::Validation,
    crate::route_error_status::RouteErrorStatus::RateLimited,
    crate::route_error_status::RouteErrorStatus::Internal,
];
pub const AUTHENTICATED_MUTATING_ROUTE_ERROR_STATUSES:
    &[crate::route_error_status::RouteErrorStatus] = &[
    crate::route_error_status::RouteErrorStatus::Authentication,
    crate::route_error_status::RouteErrorStatus::Authorization,
    crate::route_error_status::RouteErrorStatus::PayloadTooLarge,
    crate::route_error_status::RouteErrorStatus::RateLimited,
    crate::route_error_status::RouteErrorStatus::Internal,
];
pub const AUTHORIZED_MUTATING_ROUTE_ERROR_STATUSES:
    &[crate::route_error_status::RouteErrorStatus] = &[
    crate::route_error_status::RouteErrorStatus::Authentication,
    crate::route_error_status::RouteErrorStatus::Authorization,
    crate::route_error_status::RouteErrorStatus::Conflict,
    crate::route_error_status::RouteErrorStatus::PayloadTooLarge,
    crate::route_error_status::RouteErrorStatus::Validation,
    crate::route_error_status::RouteErrorStatus::RateLimited,
    crate::route_error_status::RouteErrorStatus::Internal,
];
pub const AUTHORIZED_DELETE_ROUTE_ERROR_STATUSES: &[crate::route_error_status::RouteErrorStatus] =
    &[
        crate::route_error_status::RouteErrorStatus::Authentication,
        crate::route_error_status::RouteErrorStatus::Authorization,
        crate::route_error_status::RouteErrorStatus::Conflict,
        crate::route_error_status::RouteErrorStatus::PayloadTooLarge,
        crate::route_error_status::RouteErrorStatus::RateLimited,
        crate::route_error_status::RouteErrorStatus::Internal,
    ];

#[cfg(test)]
mod tests {
    #[test]
    fn test_public_read_policy_has_stable_statuses() {
        assert_eq!(
            crate::route_error_policy::RouteErrorPolicy::Default.statuses(
                crate::authentication_requirement::AuthenticationRequirement::Public,
                crate::route_mutation::RouteMutation::ReadOnly,
            ),
            super::PUBLIC_READ_ROUTE_ERROR_STATUSES,
        );
    }
}
