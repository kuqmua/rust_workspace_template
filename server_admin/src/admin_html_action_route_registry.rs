#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[frontend_contract_macros::endpoint_registry(
    state = crate::shared_admin_auth_svc_state_arc::SharedAdminAuthSvcStateArc;
    (server_admin_contract::admin_frontend_path::AdminFrontendPath::Root, crate::root::root),
)]
pub(crate) struct AdminHtmlActionRouteRegistry;
