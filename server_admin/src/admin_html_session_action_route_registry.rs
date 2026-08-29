#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[frontend_contract_macros::endpoint_registry(
    state = crate::shared_admin_auth_svc_state_arc::SharedAdminAuthSvcStateArc;
    (server_admin_contract::admin_html_action::AdminHtmlAction::SessionRevoke, crate::revoke_session::revoke_session),
)]
pub(crate) struct AdminHtmlSessionActionRouteRegistry;
