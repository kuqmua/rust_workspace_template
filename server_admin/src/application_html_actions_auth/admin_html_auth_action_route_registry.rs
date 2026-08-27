use super::change_password::change_password;
use super::sign_in::sign_in;
use super::sign_out::sign_out;

#[path = "router.rs"]
mod router;

pub(in super::super) use router::router;

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[frontend_contract::domain_types::endpoint_registry(
    state = super::super::super::super::SharedAdminAuthSvcStateArc;
    (server_admin_contract::domain_types::AdminHtmlAction::SignIn, sign_in),
    (server_admin_contract::domain_types::AdminHtmlAction::SignOut, sign_out),
    (server_admin_contract::domain_types::AdminHtmlAction::ProfilePassword, change_password),
)]
pub(super) struct AdminHtmlAuthActionRouteRegistry;
