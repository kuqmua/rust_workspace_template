#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
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
        authentication: super::AuthenticationRequirement,
        mutation: crate::RouteMutation,
    ) -> &'static [super::RouteErrorStatus] {
        match self {
            Self::Authentication => super::PUBLIC_AUTH_ROUTE_ERROR_STATUSES,
            Self::Delete => super::AUTHORIZED_DELETE_ROUTE_ERROR_STATUSES,
            Self::ValidatedRead => super::AUTHORIZED_VALIDATED_READ_ROUTE_ERROR_STATUSES,
            Self::Default => match (authentication, mutation) {
                (super::AuthenticationRequirement::Public, crate::RouteMutation::ReadOnly) => {
                    super::PUBLIC_READ_ROUTE_ERROR_STATUSES
                }
                (super::AuthenticationRequirement::Public, crate::RouteMutation::Mutating) => {
                    super::PUBLIC_MUTATING_ROUTE_ERROR_STATUSES
                }
                (
                    super::AuthenticationRequirement::Authenticated,
                    crate::RouteMutation::ReadOnly,
                ) => super::AUTHENTICATED_READ_ROUTE_ERROR_STATUSES,
                (
                    super::AuthenticationRequirement::Authenticated,
                    crate::RouteMutation::Mutating,
                ) => super::AUTHENTICATED_MUTATING_ROUTE_ERROR_STATUSES,
                (
                    super::AuthenticationRequirement::Permission(_),
                    crate::RouteMutation::ReadOnly,
                ) => super::AUTHORIZED_READ_ROUTE_ERROR_STATUSES,
                (
                    super::AuthenticationRequirement::Permission(_),
                    crate::RouteMutation::Mutating,
                ) => super::AUTHORIZED_MUTATING_ROUTE_ERROR_STATUSES,
            },
        }
    }
}
