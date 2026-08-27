#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[frontend_contract::domain_types::endpoint_registry(
    state = super::super::super::super::super::SharedAdminAuthSvcStateArc;
    (server_admin_contract::domain_types::AdminHtmlAction::SessionRevoke, super::revoke_session),
)]
pub(in crate::domain_types::auth::html::actions) struct AdminHtmlSessionActionRouteRegistry;

impl AdminHtmlSessionActionRouteRegistry {
    pub(in crate::domain_types::auth::html::actions) fn registry_router()
    -> axum::Router<super::super::super::super::super::SharedAdminAuthSvcStateArc> {
        Self::router()
    }
}
