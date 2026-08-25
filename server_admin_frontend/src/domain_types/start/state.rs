#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub(in crate::domain_types::start) enum AdminLoadState {
    Empty(server_admin_contract::domain_types::AuthenticatedAdmin),
    Error(AdminTableLoadError),
    Loading,
    Permissions(
        server_admin_contract::domain_types::AuthenticatedAdmin,
        server_admin_contract::domain_types::AdminPermissionsPage,
    ),
    Profile(server_admin_contract::domain_types::AuthenticatedAdmin),
    Roles(
        server_admin_contract::domain_types::AuthenticatedAdmin,
        server_admin_contract::domain_types::AdminRolesPage,
    ),
    Sessions(
        server_admin_contract::domain_types::AuthenticatedAdmin,
        server_admin_contract::domain_types::AdminSessionsPage,
    ),
    Settings(
        server_admin_contract::domain_types::AuthenticatedAdmin,
        server_admin_contract::domain_types::AdminSettingsView,
    ),
    Table(
        server_admin_contract::domain_types::AuthenticatedAdmin,
        server_admin_contract::domain_types::AdminDataTableView,
    ),
    Users(
        server_admin_contract::domain_types::AuthenticatedAdmin,
        server_admin_contract::domain_types::AdminUsersPage,
    ),
}

impl AdminLoadState {
    pub(in crate::domain_types::start) const fn admin(
        &self,
    ) -> Option<&server_admin_contract::domain_types::AuthenticatedAdmin> {
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

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, thiserror::Error)]
pub(in crate::domain_types::start) enum AdminTableLoadError {
    #[error("The table request failed.")]
    Fetch,
    #[error("The server returned status {0} for {1}.")]
    Http(
        super::http::url::AdminHttpStatus,
        super::http::url::AdminCsrApiUrl,
    ),
    #[error("The table query is invalid.")]
    Query,
    #[error("The table response was invalid.")]
    Response,
}
