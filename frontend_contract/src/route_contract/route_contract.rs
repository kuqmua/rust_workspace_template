#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq)]
pub struct RouteContract {
    path: super::super::ContractStr,
    authentication: super::AuthenticationRequirement,
    method: super::HttpMethod,
    mutation: super::MutationKind,
    success_status: super::SuccessStatus,
}

impl RouteContract {
    #[must_use]
    pub const fn new(
        authentication: super::AuthenticationRequirement,
        method: super::HttpMethod,
        mutation: super::MutationKind,
        path: super::super::ContractStr,
        success_status: super::SuccessStatus,
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
    pub const fn authentication(self) -> super::AuthenticationRequirement {
        self.authentication
    }
    #[must_use]
    pub const fn method(self) -> super::HttpMethod {
        self.method
    }
    #[must_use]
    pub const fn mutation(self) -> super::MutationKind {
        self.mutation
    }
    #[must_use]
    pub const fn path(self) -> super::super::ContractStr {
        self.path
    }
    #[must_use]
    pub const fn success_status(self) -> super::SuccessStatus {
        self.success_status
    }
}
