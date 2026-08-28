#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug)]
pub(crate) struct AdminGeneratedRouteContract {
    permission: Option<crate::domain_types::StdAdminStrRef<'static>>,
    mutates: crate::domain_types::StdAdminBool,
    method: frontend_contract::domain_types::HttpMethod,
}
impl AdminGeneratedRouteContract {
    pub(super) const fn new(
        permission: Option<crate::domain_types::StdAdminStrRef<'static>>,
        mutates: crate::domain_types::StdAdminBool,
        method: frontend_contract::domain_types::HttpMethod,
    ) -> Self {
        Self {
            permission,
            mutates,
            method,
        }
    }

    pub(crate) const fn method(self) -> frontend_contract::domain_types::HttpMethod {
        self.method
    }

    pub(crate) const fn mutates(self) -> crate::domain_types::StdAdminBool {
        self.mutates
    }

    pub(crate) const fn permission(self) -> Option<crate::domain_types::StdAdminStrRef<'static>> {
        self.permission
    }
}
