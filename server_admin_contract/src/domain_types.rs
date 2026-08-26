#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "DTO implementations keep constructors adjacent to their accessors and route metadata grouped by concern"
)]

#[path = "audit_branding.rs"]
mod audit_branding;
#[path = "authorization_catalog.rs"]
mod authorization_catalog;
#[path = "identifier.rs"]
mod identifier;
#[path = "identity.rs"]
mod identity;
#[path = "table_sort.rs"]
mod table_sort;

pub use audit_branding::*;
pub use authorization_catalog::*;
pub use identifier::*;
pub use identity::*;
pub use table_sort::*;
#[path = "query.rs"]
mod query;
pub use query::{
    AdminBool, AdminDataTableFilterQuery, AdminDataTableQuery, AdminFilterField,
    AdminFilterOperationKey, AdminFilterValue, AdminPageLimit, AdminPageLimitError,
    AdminPageOffset, AdminPageTotal, AdminSortDirection, AdminTableQuery, AdminTableSearch,
    AdminTableSortKey,
};

#[path = "collections.rs"]
mod collections;
#[cfg(test)]
use collections::ADMIN_COLLECTION_MAX_ITEMS;
pub use collections::{
    AdminAuditViews, AdminCollectionError, AdminDataRows, AdminDataTables, AdminOptionalSettings,
    AdminPermissionIds, AdminPermissionSummaries, AdminPermissionValues, AdminRoleIds,
    AdminRoleNames, AdminRoleSummaries, AdminSessionViews, AdminTexts, AdminUserSummaries,
};
#[path = "dto.rs"]
mod dto;
pub use dto::{
    AdminAuditCursor, AdminAuditExport, AdminAuditExportCsv, AdminAuditPage, AdminAuditView,
    AdminChangeOwnPasswordReq, AdminCreateRoleReq, AdminCreateRoleRes, AdminCreateUserReq,
    AdminCreateUserRes, AdminDataColumn, AdminDataColumns, AdminDataFilter, AdminDataFilters,
    AdminDataInputKind, AdminDataRow, AdminDataTableCatalog, AdminDataTableView,
    AdminPermissionSummary, AdminPermissionsPage, AdminRoleSummary, AdminRolesPage,
    AdminSetRolePermissionsReq, AdminSetUserBanReq, AdminSetUserPasswordReq, AdminSetUserRolesReq,
    AdminSignInReq, AdminSignInRes, AdminUpdateRoleReq, AdminUpdateUserReq, AdminUserSummary,
    AdminUsersPage, AuthenticatedAdmin,
};

#[path = "settings.rs"]
mod settings;
pub use settings::{
    AdminBrandingView, AdminOptionalSetting, AdminSetting, AdminSettingInputKind,
    AdminSettingLabel, AdminSettingName, AdminSettingOptionality, AdminSettingSpec,
    AdminSettingsView, AdminUpdateSettingsReq,
};

#[path = "sessions.rs"]
mod sessions;
pub use sessions::{
    AdminNoBody, AdminSessionIdentifier, AdminSessionTimestamp, AdminSessionView, AdminSessionsPage,
};

#[path = "routes.rs"]
mod routes;
pub use routes::{
    AdminAuditExportRoute, AdminAuditLogRoute, AdminAuthenticationRouteFamily, AdminBrandingRoute,
    AdminChangeOwnPasswordRoute, AdminCreateRoleRoute, AdminCreateUserRoute,
    AdminDataTableFrontendPath, AdminDataTableRoute, AdminDataTablesRoute, AdminDeleteRoleRoute,
    AdminDeleteUserRoute, AdminFrontendPath, AdminHtmlAction, AdminListPermissionsRoute,
    AdminListRolesRoute, AdminListUsersRoute, AdminMeRoute, AdminPage, AdminPageCapability,
    AdminPageClientMode, AdminPageMetadata, AdminPageNavigation, AdminPagePathRef, AdminPageSpec,
    AdminRefreshRoute, AdminRevokeAllSessionsRoute, AdminRevokeSessionRoute, AdminRoute,
    AdminRoutePath, AdminRoutePathError, AdminSessionsRoute, AdminSetRolePermissionsRoute,
    AdminSetUserBanRoute, AdminSetUserPasswordRoute, AdminSetUserRolesRoute, AdminSettingsRoute,
    AdminSignInRoute, AdminSignOutRoute, AdminUpdateRoleRoute, AdminUpdateSettingsRoute,
    AdminUpdateUserRoute, admin_parameterized_route_path, audit_log_client, audit_log_route,
    branding_client, branding_route, change_own_password_client, change_own_password_route,
    create_role_client, create_role_route, create_user_client, create_user_route,
    delete_role_client, delete_role_route, delete_user_client, delete_user_route,
    export_audit_log_client, export_audit_log_route, list_data_tables_client,
    list_data_tables_route, list_permissions_client, list_permissions_route, list_roles_client,
    list_roles_route, list_users_client, list_users_route, me_client, me_route, metrics_client,
    metrics_route, open_api_client, open_api_route, read_data_table_client, read_data_table_route,
    refresh_client, refresh_route, revoke_all_sessions_client, revoke_all_sessions_route,
    revoke_session_client, revoke_session_route, sessions_client, sessions_route,
    set_role_permissions_client, set_role_permissions_route, set_user_ban_client,
    set_user_ban_route, set_user_password_client, set_user_password_route, set_user_roles_client,
    set_user_roles_route, settings_client, settings_route, sign_in_client, sign_in_route,
    sign_out_client, sign_out_route, update_role_client, update_role_route, update_settings_client,
    update_settings_route, update_user_client, update_user_route, version_client, version_route,
};
#[cfg(test)]
#[path = "domain_types__tests.rs"]
mod tests;
