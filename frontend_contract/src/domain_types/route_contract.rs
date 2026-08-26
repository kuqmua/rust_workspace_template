#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq)]
pub enum HttpMethod {
    Connect,
    Delete,
    Get,
    Head,
    Options,
    Patch,
    Post,
    Put,
    Trace,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq)]
pub enum SuccessStatus {
    Code200,
    Code201,
    Code204,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq)]
pub enum RouteErrorStatus {
    Authentication,
    Authorization,
    Conflict,
    Internal,
    MethodNotAllowed,
    PayloadTooLarge,
    RateLimited,
    ServiceUnavailable,
    Validation,
}
impl RouteErrorStatus {
    #[must_use]
    pub fn transport_status(self) -> super::TransportStatus {
        match self {
            Self::Authentication => {
                super::TransportStatus::from(super::KnownHttpStatus::Unauthorized)
            }
            Self::Authorization => super::TransportStatus::from(super::KnownHttpStatus::Forbidden),
            Self::Conflict => super::TransportStatus::from(super::KnownHttpStatus::Conflict),
            Self::Internal => {
                super::TransportStatus::from(super::KnownHttpStatus::InternalServerError)
            }
            Self::MethodNotAllowed => {
                super::TransportStatus::from(super::KnownHttpStatus::MethodNotAllowed)
            }
            Self::PayloadTooLarge => {
                super::TransportStatus::from(super::KnownHttpStatus::PayloadTooLarge)
            }
            Self::RateLimited => {
                super::TransportStatus::from(super::KnownHttpStatus::TooManyRequests)
            }
            Self::ServiceUnavailable => {
                super::TransportStatus::from(super::KnownHttpStatus::ServiceUnavailable)
            }
            Self::Validation => {
                super::TransportStatus::from(super::KnownHttpStatus::UnprocessableEntity)
            }
        }
    }
}
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
        authentication: AuthenticationRequirement,
        mutation: super::RouteMutation,
    ) -> &'static [RouteErrorStatus] {
        match self {
            Self::Authentication => PUBLIC_AUTH_ROUTE_ERROR_STATUSES,
            Self::Delete => AUTHORIZED_DELETE_ROUTE_ERROR_STATUSES,
            Self::ValidatedRead => AUTHORIZED_VALIDATED_READ_ROUTE_ERROR_STATUSES,
            Self::Default => match (authentication, mutation) {
                (AuthenticationRequirement::Public, super::RouteMutation::ReadOnly) => {
                    PUBLIC_READ_ROUTE_ERROR_STATUSES
                }
                (AuthenticationRequirement::Public, super::RouteMutation::Mutating) => {
                    PUBLIC_MUTATING_ROUTE_ERROR_STATUSES
                }
                (AuthenticationRequirement::Authenticated, super::RouteMutation::ReadOnly) => {
                    AUTHENTICATED_READ_ROUTE_ERROR_STATUSES
                }
                (AuthenticationRequirement::Authenticated, super::RouteMutation::Mutating) => {
                    AUTHENTICATED_MUTATING_ROUTE_ERROR_STATUSES
                }
                (AuthenticationRequirement::Permission(_), super::RouteMutation::ReadOnly) => {
                    AUTHORIZED_READ_ROUTE_ERROR_STATUSES
                }
                (AuthenticationRequirement::Permission(_), super::RouteMutation::Mutating) => {
                    AUTHORIZED_MUTATING_ROUTE_ERROR_STATUSES
                }
            },
        }
    }
}
impl SuccessStatus {
    #[must_use]
    pub fn transport_status(self) -> super::TransportStatus {
        match self {
            Self::Code200 => super::TransportStatus::from(super::KnownHttpStatus::Ok),
            Self::Code201 => super::TransportStatus::from(super::KnownHttpStatus::Created),
            Self::Code204 => super::TransportStatus::from(super::KnownHttpStatus::NoContent),
        }
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuthenticationRequirement {
    Authenticated,
    Permission(super::ContractStr),
    Public,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq)]
pub enum MutationKind {
    ReadOnly,
    Mutating,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq)]
pub enum OperationKind {
    CreateMany,
    CreateOne,
    DeleteMany,
    DeleteOne,
    ReadMany,
    ReadOne,
    UpdateMany,
    UpdateOne,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConfirmationRequirement {
    NotRequired,
    Required,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActionContract {
    route: RouteContract,
    confirmation: ConfirmationRequirement,
    operation: OperationKind,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    PartialEq,
    Eq,
    newtype::AsRefTarget,
    newtype::FromInner,
)]
pub struct ActionContracts(
    bounded_types::domain_types::vector::BoundedVec<ActionContract, 0, { usize::MAX }>,
);
impl TryFrom<Vec<ActionContract>> for ActionContracts {
    type Error = bounded_types::domain_types::BoundedValueError;
    fn try_from(value: Vec<ActionContract>) -> Result<Self, Self::Error> {
        bounded_types::domain_types::vector::BoundedVec::try_from_collection_vec(value)
            .map(Self::from)
    }
}
impl ActionContracts {
    #[must_use]
    pub fn from_max_iter<Values>(values: Values) -> Self
    where
        Values: IntoIterator<Item = ActionContract>,
    {
        Self::from(bounded_types::domain_types::vector::BoundedVec::from_max_iter(values))
    }
}
impl ActionContract {
    #[must_use]
    pub const fn new(operation: OperationKind, route: RouteContract) -> Self {
        Self {
            confirmation: ConfirmationRequirement::NotRequired,
            operation,
            route,
        }
    }
    #[must_use]
    pub const fn confirmation(self) -> ConfirmationRequirement {
        self.confirmation
    }
    #[must_use]
    pub const fn operation(self) -> OperationKind {
        self.operation
    }
    #[must_use]
    pub const fn route(self) -> RouteContract {
        self.route
    }
    #[must_use]
    pub const fn with_confirmation(mut self, value: ConfirmationRequirement) -> Self {
        self.confirmation = value;
        self
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq)]
pub struct RouteContract {
    path: super::ContractStr,
    authentication: AuthenticationRequirement,
    method: HttpMethod,
    mutation: MutationKind,
    success_status: SuccessStatus,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    PartialEq,
    Eq,
    newtype::AsRefTarget,
    newtype::FromInner,
)]
pub struct RouteContracts(
    bounded_types::domain_types::vector::BoundedVec<RouteContract, 0, { usize::MAX }>,
);
impl TryFrom<Vec<RouteContract>> for RouteContracts {
    type Error = bounded_types::domain_types::BoundedValueError;
    fn try_from(value: Vec<RouteContract>) -> Result<Self, Self::Error> {
        bounded_types::domain_types::vector::BoundedVec::try_from_collection_vec(value)
            .map(Self::from)
    }
}
impl RouteContracts {
    #[must_use]
    pub fn from_max_iter<Values>(values: Values) -> Self
    where
        Values: IntoIterator<Item = RouteContract>,
    {
        Self::from(bounded_types::domain_types::vector::BoundedVec::from_max_iter(values))
    }
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
