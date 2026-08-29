use super::create_role::create_role;
use super::delete_role::delete_role;
use super::role_permissions::role_permissions;
use super::update_role::update_role;

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[frontend_contract::endpoint_registry(
    state = crate::SharedAdminAuthSvcStateArc;
    (server_admin_contract::domain_types::AdminHtmlAction::RoleCreate, create_role),
    (server_admin_contract::domain_types::AdminHtmlAction::RoleUpdate, update_role),
    (server_admin_contract::domain_types::AdminHtmlAction::RoleDelete, delete_role),
    (server_admin_contract::domain_types::AdminHtmlAction::RolePermissions, role_permissions),
)]
pub(crate) struct AdminHtmlRoleActionRouteRegistry;
