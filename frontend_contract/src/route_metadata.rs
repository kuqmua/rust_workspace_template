use super::RouteMethod;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct RouteMetadata {
    authentication: crate::domain_types::AuthenticationRequirement,
    error_statuses: &'static [crate::domain_types::RouteErrorStatus],
    openapi_operation_id: crate::domain_types::ContractStr,
    path: crate::domain_types::ContractStr,
    method: RouteMethod,
    mutation: crate::domain_types::RouteMutation,
    success_status: crate::domain_types::SuccessStatus,
}
impl RouteMetadata {
    #[must_use]
    pub const fn new(
        method: RouteMethod,
        openapi_operation_id: crate::domain_types::ContractStr,
        path: crate::domain_types::ContractStr,
    ) -> Self {
        Self::new_with_policy(
            crate::domain_types::AuthenticationRequirement::Public,
            &[],
            method,
            crate::domain_types::RouteMutation::ReadOnly,
            openapi_operation_id,
            path,
            crate::domain_types::SuccessStatus::Code200,
        )
    }
    #[must_use]
    pub const fn new_with_policy(
        authentication: crate::domain_types::AuthenticationRequirement,
        error_statuses: &'static [crate::domain_types::RouteErrorStatus],
        method: RouteMethod,
        mutation: crate::domain_types::RouteMutation,
        openapi_operation_id: crate::domain_types::ContractStr,
        path: crate::domain_types::ContractStr,
        success_status: crate::domain_types::SuccessStatus,
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
    pub const fn authentication(self) -> crate::domain_types::AuthenticationRequirement {
        self.authentication
    }
    #[must_use]
    pub const fn error_statuses(self) -> &'static [crate::domain_types::RouteErrorStatus] {
        self.error_statuses
    }
    #[must_use]
    pub const fn access(self) -> crate::domain_types::RouteAccess {
        match self.authentication {
            crate::domain_types::AuthenticationRequirement::Public => {
                crate::domain_types::RouteAccess::Public
            }
            crate::domain_types::AuthenticationRequirement::Authenticated
            | crate::domain_types::AuthenticationRequirement::Permission(_) => {
                crate::domain_types::RouteAccess::Authenticated
            }
        }
    }
    #[must_use]
    pub fn method(self) -> crate::domain_types::ContractStr {
        self.method.as_str()
    }
    #[must_use]
    pub const fn route_method(self) -> RouteMethod {
        self.method
    }
    #[must_use]
    pub const fn openapi_operation_id(self) -> crate::domain_types::ContractStr {
        self.openapi_operation_id
    }
    #[must_use]
    pub const fn path(self) -> crate::domain_types::ContractStr {
        self.path
    }
    #[must_use]
    pub const fn mutation(self) -> crate::domain_types::RouteMutation {
        self.mutation
    }
    #[must_use]
    pub const fn success_status(self) -> crate::domain_types::SuccessStatus {
        self.success_status
    }
    #[must_use]
    pub const fn contract(self) -> crate::domain_types::RouteContract {
        crate::domain_types::RouteContract::new(
            self.authentication,
            match self.method {
                RouteMethod::Connect => crate::domain_types::HttpMethod::Connect,
                RouteMethod::Delete => crate::domain_types::HttpMethod::Delete,
                RouteMethod::Get => crate::domain_types::HttpMethod::Get,
                RouteMethod::Head => crate::domain_types::HttpMethod::Head,
                RouteMethod::Options => crate::domain_types::HttpMethod::Options,
                RouteMethod::Patch => crate::domain_types::HttpMethod::Patch,
                RouteMethod::Post => crate::domain_types::HttpMethod::Post,
                RouteMethod::Put => crate::domain_types::HttpMethod::Put,
                RouteMethod::Trace => crate::domain_types::HttpMethod::Trace,
            },
            match self.mutation {
                crate::domain_types::RouteMutation::ReadOnly => {
                    crate::domain_types::MutationKind::ReadOnly
                }
                crate::domain_types::RouteMutation::Mutating => {
                    crate::domain_types::MutationKind::Mutating
                }
            },
            self.path,
            self.success_status,
        )
    }
}
