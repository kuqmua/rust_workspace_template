proc_macro_frontend_contract_route_registry::route_registry! {
    pub(crate);
    state = crate::shared_admin_auth_service_state_arc::SharedAdminAuthServiceStateArc;
    (server_admin_contract::admin_html_action::AdminHtmlAction::UserCreate, crate::create_user::create_user),
    (server_admin_contract::admin_html_action::AdminHtmlAction::UserUpdate, crate::update_user::update_user),
    (server_admin_contract::admin_html_action::AdminHtmlAction::UserRecordUpdate, crate::update_user_record::update_user_record),
    (server_admin_contract::admin_html_action::AdminHtmlAction::UserPassword, crate::user_password::user_password),
    (server_admin_contract::admin_html_action::AdminHtmlAction::UserBan, crate::user_ban::user_ban),
    (server_admin_contract::admin_html_action::AdminHtmlAction::UserDelete, crate::delete_user::delete_user),
    (server_admin_contract::admin_html_action::AdminHtmlAction::UserRoles, crate::user_roles::user_roles),
}
