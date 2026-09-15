proc_macro_frontend_contract_route_registry::route_registry! {
    pub(crate);
    state = crate::shared_admin_auth_svc_state_arc::SharedAdminAuthSvcStateArc;
    (server_admin_contract::admin_frontend_path::AdminFrontendPath::OpenApi, crate::admin_html_open_api::admin_html_open_api),
}
