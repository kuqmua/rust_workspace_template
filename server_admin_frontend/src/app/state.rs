#[derive(optml::Optml, Clone, Debug)]
pub(in crate::app) enum AdminLoadState {
    Empty(server_admin_contract::AuthenticatedAdmin),
    Error(AdminTableLoadError),
    Loading,
    Permissions(
        server_admin_contract::AuthenticatedAdmin,
        server_admin_contract::AdminPermissionsPage,
    ),
    Profile(server_admin_contract::AuthenticatedAdmin),
    Roles(
        server_admin_contract::AuthenticatedAdmin,
        server_admin_contract::AdminRolesPage,
    ),
    Sessions(
        server_admin_contract::AuthenticatedAdmin,
        server_admin_contract::AdminSessionsPage,
    ),
    Settings(
        server_admin_contract::AuthenticatedAdmin,
        server_admin_contract::AdminSettingsView,
    ),
    Table(
        server_admin_contract::AuthenticatedAdmin,
        server_admin_contract::AdminDataTableView,
    ),
    Users(
        server_admin_contract::AuthenticatedAdmin,
        server_admin_contract::AdminUsersPage,
    ),
}

impl AdminLoadState {
    pub(in crate::app) const fn admin(&self) -> Option<&server_admin_contract::AuthenticatedAdmin> {
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

#[derive(optml::Optml, Clone, Debug, thiserror::Error)]
pub(in crate::app) enum AdminTableLoadError {
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
