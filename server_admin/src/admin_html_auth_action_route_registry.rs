use crate::change_password::change_password;
use crate::sign_in::sign_in;
use crate::sign_out::sign_out;

pub(crate) use admin_html_auth_action_router::admin_html_auth_action_router;

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[frontend_contract::domain_types::endpoint_registry(
    state = crate::SharedAdminAuthSvcStateArc;
    (server_admin_contract::domain_types::AdminHtmlAction::SignIn, sign_in),
    (server_admin_contract::domain_types::AdminHtmlAction::SignOut, sign_out),
    (server_admin_contract::domain_types::AdminHtmlAction::ProfilePassword, change_password),
)]
pub(crate) struct AdminHtmlAuthActionRouteRegistry;

// Root-owned module compatibility wrappers.
mod admin_html_auth_action_router {
    pub use crate::admin_html_auth_action_router::*;
}
