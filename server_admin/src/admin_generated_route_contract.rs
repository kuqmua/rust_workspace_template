#[derive(proc_macro_getters::Getters, proc_macro_new::New)]
#[getters(bare)]
#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug)]
pub(crate) struct AdminGeneratedRouteContract {
    #[getters(copy)]
    permission: Option<server_admin_core::std_admin_str_ref::StdAdminStrRef<'static>>,
    #[getters(copy)]
    mutates: server_admin_core::std_admin_bool::StdAdminBool,
    #[getters(copy)]
    method: frontend_contract::route_method::RouteMethod,
}
