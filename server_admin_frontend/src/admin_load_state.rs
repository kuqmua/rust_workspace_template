#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub(crate) enum AdminLoadState {
    Empty(server_admin_contract::authenticated_admin::AuthenticatedAdmin),
    Error(super::admin_table_load_error::AdminTableLoadError),
    Loading,
    Permissions(
        server_admin_contract::authenticated_admin::AuthenticatedAdmin,
        server_admin_contract::admin_permissions_page::AdminPermissionsPage,
    ),
    Profile(server_admin_contract::authenticated_admin::AuthenticatedAdmin),
    Roles(
        server_admin_contract::authenticated_admin::AuthenticatedAdmin,
        server_admin_contract::admin_roles_page::AdminRolesPage,
    ),
    Sessions(
        server_admin_contract::authenticated_admin::AuthenticatedAdmin,
        server_admin_contract::admin_sessions_page::AdminSessionsPage,
    ),
    Settings(
        server_admin_contract::authenticated_admin::AuthenticatedAdmin,
        server_admin_contract::admin_settings_view::AdminSettingsView,
    ),
    Table(
        server_admin_contract::authenticated_admin::AuthenticatedAdmin,
        server_admin_contract::admin_data_table_view::AdminDataTableView,
    ),
    Users(
        server_admin_contract::authenticated_admin::AuthenticatedAdmin,
        server_admin_contract::admin_users_page::AdminUsersPage,
    ),
}

impl AdminLoadState {
    pub(crate) const fn admin(
        &self,
    ) -> Option<&server_admin_contract::authenticated_admin::AuthenticatedAdmin> {
        match self {
            Self::Permissions(admin, _)
            | Self::Roles(admin, _)
            | Self::Sessions(admin, _)
            | Self::Settings(admin, _)
            | Self::Table(admin, _)
            | Self::Users(admin, _)
            | Self::Empty(admin)
            | Self::Profile(admin) => Some(admin),
            Self::Error(_) | Self::Loading => None,
        }
    }
}
