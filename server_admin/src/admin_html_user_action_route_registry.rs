#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[frontend_contract_macros::endpoint_registry(
    state = crate::shared_admin_auth_svc_state_arc::SharedAdminAuthSvcStateArc;
    (server_admin_contract::admin_html_action::AdminHtmlAction::UserCreate, crate::create_user::create_user),
    (server_admin_contract::admin_html_action::AdminHtmlAction::UserUpdate, crate::update_user::update_user),
    (server_admin_contract::admin_html_action::AdminHtmlAction::UserPassword, crate::user_password::user_password),
    (server_admin_contract::admin_html_action::AdminHtmlAction::UserBan, crate::user_ban::user_ban),
    (server_admin_contract::admin_html_action::AdminHtmlAction::UserDelete, crate::delete_user::delete_user),
    (server_admin_contract::admin_html_action::AdminHtmlAction::UserRoles, crate::user_roles::user_roles),
)]
pub(crate) struct AdminHtmlUserActionRouteRegistry;
