use super::create_role::create_role;
use super::delete_role::delete_role;
use super::role_permissions::role_permissions;
use super::update_role::update_role;

#[path = "router.rs"]
mod router;

pub(in super::super) use router::router;

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[frontend_contract::domain_types::endpoint_registry(
    state = super::super::super::super::SharedAdminAuthSvcStateArc;
    (server_admin_contract::domain_types::AdminHtmlAction::RoleCreate, create_role),
    (server_admin_contract::domain_types::AdminHtmlAction::RoleUpdate, update_role),
    (server_admin_contract::domain_types::AdminHtmlAction::RoleDelete, delete_role),
    (server_admin_contract::domain_types::AdminHtmlAction::RolePermissions, role_permissions),
)]
struct AdminHtmlRoleActionRouteRegistry;
