proc_macro_frontend_contract_endpoint_registry::endpoint_registry! {
    pub(crate);
    state = crate::shared_admin_auth_svc_state_arc::SharedAdminAuthSvcStateArc;
    (server_admin_contract::admin_html_action::AdminHtmlAction::SettingsUpdate, crate::update_settings::update_settings),
}
