use crate::admin_html_open_api;

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[frontend_contract::endpoint_registry(
    state = crate::SharedAdminAuthSvcStateArc;
    (server_admin_contract::domain_types::AdminFrontendPath::OpenApi, admin_html_open_api),
)]
pub(crate) struct AdminHtmlSwaggerRouteRegistry;
