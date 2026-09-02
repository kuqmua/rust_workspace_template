proc_macro_frontend_contract::endpoint_registry! {
    pub(crate);
    state = crate::shared_admin_auth_svc_state_arc::SharedAdminAuthSvcStateArc;
    (server_admin_contract::admin_html_action::AdminHtmlAction::RoleCreate, crate::create_role::create_role),
    (server_admin_contract::admin_html_action::AdminHtmlAction::RoleUpdate, crate::update_role::update_role),
    (server_admin_contract::admin_html_action::AdminHtmlAction::RoleDelete, crate::delete_role::delete_role),
    (server_admin_contract::admin_html_action::AdminHtmlAction::RolePermissions, crate::role_permissions::role_permissions),
}
