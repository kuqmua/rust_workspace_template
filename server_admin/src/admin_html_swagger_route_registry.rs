#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[frontend_contract_macros::endpoint_registry(
    state = crate::shared_admin_auth_svc_state_arc::SharedAdminAuthSvcStateArc;
    (server_admin_contract::admin_frontend_path::AdminFrontendPath::OpenApi, crate::admin_html_open_api::admin_html_open_api),
)]
pub(crate) struct AdminHtmlSwaggerRouteRegistry;
