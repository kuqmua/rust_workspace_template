frontend_contract_macros::endpoint_registry! {
    pub(crate);
    state = crate::shared_admin_auth_svc_state_arc::SharedAdminAuthSvcStateArc;
    (server_admin_contract::admin_frontend_path::AdminFrontendPath::SignIn, crate::sign_in_page::sign_in_page),
    (server_admin_contract::admin_frontend_path::AdminFrontendPath::Tables, crate::data_tables::data_tables),
    (server_admin_contract::admin_frontend_path::AdminFrontendPath::Users, crate::users::users),
    (server_admin_contract::admin_frontend_path::AdminFrontendPath::UsersCreate, crate::users_create_page::users_create_page),
    (server_admin_contract::admin_frontend_path::AdminFrontendPath::UsersManage, crate::users_manage_page::users_manage_page),
    (server_admin_contract::admin_frontend_path::AdminFrontendPath::Roles, crate::roles::roles),
    (server_admin_contract::admin_frontend_path::AdminFrontendPath::RolesCreate, crate::roles_create_page::roles_create_page),
    (server_admin_contract::admin_frontend_path::AdminFrontendPath::RolesManage, crate::roles_manage_page::roles_manage_page),
    (server_admin_contract::admin_frontend_path::AdminFrontendPath::Permissions, crate::permissions::permissions),
    (server_admin_contract::admin_frontend_path::AdminFrontendPath::Sessions, crate::admin_html_sessions_page::admin_html_sessions_page),
    (server_admin_contract::admin_frontend_path::AdminFrontendPath::Profile, crate::profile::profile),
    (server_admin_contract::admin_frontend_path::AdminFrontendPath::Settings, crate::settings::settings),
    (server_admin_contract::admin_frontend_path::AdminFrontendPath::Version, crate::version::version),
}
