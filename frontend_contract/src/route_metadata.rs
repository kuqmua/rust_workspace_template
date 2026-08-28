use super::RouteMethod;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct RouteMetadata {
    authentication: crate::AuthenticationRequirement,
    error_statuses: &'static [crate::RouteErrorStatus],
    openapi_operation_id: crate::ContractStr,
    path: crate::ContractStr,
    method: RouteMethod,
    mutation: crate::RouteMutation,
    success_status: crate::SuccessStatus,
}
impl RouteMetadata {
    #[must_use]
    pub const fn new(
        method: RouteMethod,
        openapi_operation_id: crate::ContractStr,
        path: crate::ContractStr,
    ) -> Self {
        Self::new_with_policy(
            crate::AuthenticationRequirement::Public,
            &[],
            method,
            crate::RouteMutation::ReadOnly,
            openapi_operation_id,
            path,
            crate::SuccessStatus::Code200,
        )
    }
    #[must_use]
    pub const fn new_with_policy(
        authentication: crate::AuthenticationRequirement,
        error_statuses: &'static [crate::RouteErrorStatus],
        method: RouteMethod,
        mutation: crate::RouteMutation,
        openapi_operation_id: crate::ContractStr,
        path: crate::ContractStr,
        success_status: crate::SuccessStatus,
    ) -> Self {
        Self {
            authentication,
            error_statuses,
            openapi_operation_id,
            path,
            method,
            mutation,
            success_status,
        }
    }
    #[must_use]
    pub const fn authentication(self) -> crate::AuthenticationRequirement {
        self.authentication
    }
    #[must_use]
    pub const fn error_statuses(self) -> &'static [crate::RouteErrorStatus] {
        self.error_statuses
    }
    #[must_use]
    pub const fn access(self) -> crate::RouteAccess {
        match self.authentication {
            crate::AuthenticationRequirement::Public => crate::RouteAccess::Public,
            crate::AuthenticationRequirement::Authenticated
            | crate::AuthenticationRequirement::Permission(_) => crate::RouteAccess::Authenticated,
        }
    }
    #[must_use]
    pub fn method(self) -> crate::ContractStr {
        self.method.as_str()
    }
    #[must_use]
    pub const fn route_method(self) -> RouteMethod {
        self.method
    }
    #[must_use]
    pub const fn openapi_operation_id(self) -> crate::ContractStr {
        self.openapi_operation_id
    }
    #[must_use]
    pub const fn path(self) -> crate::ContractStr {
        self.path
    }
    #[must_use]
    pub const fn mutation(self) -> crate::RouteMutation {
        self.mutation
    }
    #[must_use]
    pub const fn success_status(self) -> crate::SuccessStatus {
        self.success_status
    }
    #[must_use]
    pub const fn contract(self) -> crate::RouteContract {
        crate::RouteContract::new(
            self.authentication,
            self.method,
            match self.mutation {
                crate::RouteMutation::ReadOnly => crate::MutationKind::ReadOnly,
                crate::RouteMutation::Mutating => crate::MutationKind::Mutating,
            },
            self.path,
            self.success_status,
        )
    }
}
