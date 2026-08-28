#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[frontend_contract::domain_types::endpoint_registry(
    state = crate::SharedAdminAuthSvcStateArc;
    (server_admin_contract::domain_types::AdminFrontendPath::Root, crate::root),
)]
pub(crate) struct AdminHtmlActionRouteRegistry;
