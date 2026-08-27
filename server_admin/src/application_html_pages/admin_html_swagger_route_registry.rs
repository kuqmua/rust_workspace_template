use super::open_api;

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[frontend_contract::domain_types::endpoint_registry(
    state = super::super::super::SharedAdminAuthSvcStateArc;
    (server_admin_contract::domain_types::AdminFrontendPath::OpenApi, open_api),
)]
pub(super) struct AdminHtmlSwaggerRouteRegistry;
