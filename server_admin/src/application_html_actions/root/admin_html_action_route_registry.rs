#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[frontend_contract::domain_types::endpoint_registry(
    state = super::super::super::super::SharedAdminAuthSvcStateArc;
    (server_admin_contract::domain_types::AdminFrontendPath::Root, super::root),
)]
pub(in crate::domain_types::auth::html) struct AdminHtmlActionRouteRegistry;

impl AdminHtmlActionRouteRegistry {
    pub(in crate::domain_types::auth::html) fn registry_router()
    -> axum::Router<super::super::super::super::SharedAdminAuthSvcStateArc> {
        Self::router()
    }
}
