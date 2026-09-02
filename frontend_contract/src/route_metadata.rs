#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq,
)]
pub struct RouteMetadata {
    #[getters(copy)]
    authentication: crate::authentication_requirement::AuthenticationRequirement,
    #[getters(copy)]
    error_statuses: &'static [crate::route_error_status::RouteErrorStatus],
    #[getters(copy)]
    openapi_operation_id: crate::contract_str::ContractStr,
    #[getters(copy)]
    path: crate::contract_str::ContractStr,
    #[getters(skip)]
    method: crate::route_method::RouteMethod,
    #[getters(copy)]
    mutation: crate::route_mutation::RouteMutation,
    #[getters(copy)]
    success_status: crate::success_status::SuccessStatus,
}
impl RouteMetadata {
    #[must_use]
    pub const fn new(
        method: crate::route_method::RouteMethod,
        openapi_operation_id: crate::contract_str::ContractStr,
        path: crate::contract_str::ContractStr,
    ) -> Self {
        Self::new_with_policy(
            crate::authentication_requirement::AuthenticationRequirement::Public,
            &[],
            method,
            crate::route_mutation::RouteMutation::ReadOnly,
            openapi_operation_id,
            path,
            crate::success_status::SuccessStatus::Code200,
        )
    }
    #[must_use]
    pub const fn new_with_policy(
        authentication: crate::authentication_requirement::AuthenticationRequirement,
        error_statuses: &'static [crate::route_error_status::RouteErrorStatus],
        method: crate::route_method::RouteMethod,
        mutation: crate::route_mutation::RouteMutation,
        openapi_operation_id: crate::contract_str::ContractStr,
        path: crate::contract_str::ContractStr,
        success_status: crate::success_status::SuccessStatus,
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
    pub const fn access(self) -> crate::route_access::RouteAccess {
        match self.authentication {
            crate::authentication_requirement::AuthenticationRequirement::Public => {
                crate::route_access::RouteAccess::Public
            }
            crate::authentication_requirement::AuthenticationRequirement::Authenticated
            | crate::authentication_requirement::AuthenticationRequirement::Permission(_) => {
                crate::route_access::RouteAccess::Authenticated
            }
        }
    }
    #[must_use]
    pub fn method(self) -> crate::contract_str::ContractStr {
        self.method.as_str()
    }
    #[must_use]
    pub const fn route_method(self) -> crate::route_method::RouteMethod {
        self.method
    }

    #[must_use]
    pub const fn contract(self) -> crate::route_contract::RouteContract {
        crate::route_contract::RouteContract::new(
            self.authentication,
            self.method,
            match self.mutation {
                crate::route_mutation::RouteMutation::ReadOnly => {
                    crate::mutation_kind::MutationKind::ReadOnly
                }
                crate::route_mutation::RouteMutation::Mutating => {
                    crate::mutation_kind::MutationKind::Mutating
                }
            },
            self.path,
            self.success_status,
        )
    }
}
