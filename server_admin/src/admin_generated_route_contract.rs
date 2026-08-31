#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, generate_accessor::Getters,
)]
pub(crate) struct AdminGeneratedRouteContract {
    permission: Option<server_admin_core::std_admin_str_ref::StdAdminStrRef<'static>>,
    mutates: server_admin_core::std_admin_bool::StdAdminBool,
    method: frontend_contract::route_method::RouteMethod,
}
impl AdminGeneratedRouteContract {
    pub(crate) const fn new(
        permission: Option<server_admin_core::std_admin_str_ref::StdAdminStrRef<'static>>,
        mutates: server_admin_core::std_admin_bool::StdAdminBool,
        method: frontend_contract::route_method::RouteMethod,
    ) -> Self {
        Self {
            permission,
            mutates,
            method,
        }
    }

    pub(crate) const fn method(self) -> frontend_contract::route_method::RouteMethod {
        self.method
    }

    pub(crate) const fn mutates(self) -> server_admin_core::std_admin_bool::StdAdminBool {
        self.mutates
    }

    pub(crate) const fn permission(
        self,
    ) -> Option<server_admin_core::std_admin_str_ref::StdAdminStrRef<'static>> {
        self.permission
    }
}
