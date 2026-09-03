proc_macro_frontend_contract_endpoint_registry::endpoint_registry! {
    pub(crate);
    state = crate::shared_admin_auth_svc_state_arc::SharedAdminAuthSvcStateArc;
    (server_admin_contract::admin_html_action::AdminHtmlAction::SignIn, crate::sign_in::sign_in),
    (server_admin_contract::admin_html_action::AdminHtmlAction::SignOut, crate::sign_out::sign_out),
    (server_admin_contract::admin_html_action::AdminHtmlAction::ProfilePassword, crate::change_password::change_password),
}
