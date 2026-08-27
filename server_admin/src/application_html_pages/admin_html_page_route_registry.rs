use super::{
    data_tables, permissions, profile, roles, roles_create_page, roles_manage_page, sessions,
    settings, sign_in_page, users, users_create_page, users_manage_page, version,
};

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[frontend_contract::domain_types::endpoint_registry(
    state = super::super::super::SharedAdminAuthSvcStateArc;
    (server_admin_contract::domain_types::AdminFrontendPath::SignIn, sign_in_page),
    (server_admin_contract::domain_types::AdminFrontendPath::Tables, data_tables),
    (server_admin_contract::domain_types::AdminFrontendPath::Users, users),
    (server_admin_contract::domain_types::AdminFrontendPath::UsersCreate, users_create_page),
    (server_admin_contract::domain_types::AdminFrontendPath::UsersManage, users_manage_page),
    (server_admin_contract::domain_types::AdminFrontendPath::Roles, roles),
    (server_admin_contract::domain_types::AdminFrontendPath::RolesCreate, roles_create_page),
    (server_admin_contract::domain_types::AdminFrontendPath::RolesManage, roles_manage_page),
    (server_admin_contract::domain_types::AdminFrontendPath::Permissions, permissions),
    (server_admin_contract::domain_types::AdminFrontendPath::Sessions, sessions),
    (server_admin_contract::domain_types::AdminFrontendPath::Profile, profile),
    (server_admin_contract::domain_types::AdminFrontendPath::Settings, settings),
    (server_admin_contract::domain_types::AdminFrontendPath::Version, version),
)]
pub(super) struct AdminHtmlPageRouteRegistry;
