#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq,
)]
pub enum RouteErrorPolicy {
    Authentication,
    Default,
    Delete,
    ValidatedRead,
}

impl RouteErrorPolicy {
    #[must_use]
    pub const fn statuses(
        self,
        authentication: crate::authentication_requirement::AuthenticationRequirement,
        mutation: crate::route_mutation::RouteMutation,
    ) -> &'static [crate::route_error_status::RouteErrorStatus] {
        match self {
            Self::Authentication => crate::route_contract::PUBLIC_AUTH_ROUTE_ERROR_STATUSES,
            Self::Delete => crate::route_contract::AUTHORIZED_DELETE_ROUTE_ERROR_STATUSES,
            Self::ValidatedRead => {
                crate::route_contract::AUTHORIZED_VALIDATED_READ_ROUTE_ERROR_STATUSES
            }
            Self::Default => match (authentication, mutation) {
                (
                    crate::authentication_requirement::AuthenticationRequirement::Public,
                    crate::route_mutation::RouteMutation::ReadOnly,
                ) => crate::route_contract::PUBLIC_READ_ROUTE_ERROR_STATUSES,
                (
                    crate::authentication_requirement::AuthenticationRequirement::Public,
                    crate::route_mutation::RouteMutation::Mutating,
                ) => crate::route_contract::PUBLIC_MUTATING_ROUTE_ERROR_STATUSES,
                (
                    crate::authentication_requirement::AuthenticationRequirement::Authenticated,
                    crate::route_mutation::RouteMutation::ReadOnly,
                ) => crate::route_contract::AUTHENTICATED_READ_ROUTE_ERROR_STATUSES,
                (
                    crate::authentication_requirement::AuthenticationRequirement::Authenticated,
                    crate::route_mutation::RouteMutation::Mutating,
                ) => crate::route_contract::AUTHENTICATED_MUTATING_ROUTE_ERROR_STATUSES,
                (
                    crate::authentication_requirement::AuthenticationRequirement::Permission(_),
                    crate::route_mutation::RouteMutation::ReadOnly,
                ) => crate::route_contract::AUTHORIZED_READ_ROUTE_ERROR_STATUSES,
                (
                    crate::authentication_requirement::AuthenticationRequirement::Permission(_),
                    crate::route_mutation::RouteMutation::Mutating,
                ) => crate::route_contract::AUTHORIZED_MUTATING_ROUTE_ERROR_STATUSES,
            },
        }
    }
}
