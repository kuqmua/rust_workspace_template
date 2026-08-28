#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[frontend_contract::domain_types::endpoint_registry(
    state = crate::SharedAdminAuthSvcStateArc;
    (server_admin_contract::domain_types::AdminHtmlAction::SessionRevoke, crate::revoke_session),
)]
pub(crate) struct AdminHtmlSessionActionRouteRegistry;

impl AdminHtmlSessionActionRouteRegistry {
    pub(crate) fn registry_router() -> axum::Router<crate::SharedAdminAuthSvcStateArc> {
        Self::router()
    }
}
