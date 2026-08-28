use crate::{
    admin_html_sessions_page, data_tables, permissions, profile, roles, roles_create_page,
    roles_manage_page, settings, sign_in_page, users, users_create_page, users_manage_page,
    version,
};

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[frontend_contract::endpoint_registry(
    state = crate::SharedAdminAuthSvcStateArc;
    (server_admin_contract::domain_types::AdminFrontendPath::SignIn, sign_in_page),
    (server_admin_contract::domain_types::AdminFrontendPath::Tables, data_tables),
    (server_admin_contract::domain_types::AdminFrontendPath::Users, users),
    (server_admin_contract::domain_types::AdminFrontendPath::UsersCreate, users_create_page),
    (server_admin_contract::domain_types::AdminFrontendPath::UsersManage, users_manage_page),
    (server_admin_contract::domain_types::AdminFrontendPath::Roles, roles),
    (server_admin_contract::domain_types::AdminFrontendPath::RolesCreate, roles_create_page),
    (server_admin_contract::domain_types::AdminFrontendPath::RolesManage, roles_manage_page),
    (server_admin_contract::domain_types::AdminFrontendPath::Permissions, permissions),
    (server_admin_contract::domain_types::AdminFrontendPath::Sessions, admin_html_sessions_page),
    (server_admin_contract::domain_types::AdminFrontendPath::Profile, profile),
    (server_admin_contract::domain_types::AdminFrontendPath::Settings, settings),
    (server_admin_contract::domain_types::AdminFrontendPath::Version, version),
)]
pub(crate) struct AdminHtmlPageRouteRegistry;
