use super::create_role::create_role;
use super::delete_role::delete_role;
use super::role_permissions::role_permissions;
use super::update_role::update_role;

#[path = "admin_html_role_action_router.rs"]
mod admin_html_role_action_router;

pub(in super::super) use admin_html_role_action_router::admin_html_role_action_router;

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[frontend_contract::domain_types::endpoint_registry(
    state = super::super::super::super::SharedAdminAuthSvcStateArc;
    (server_admin_contract::domain_types::AdminHtmlAction::RoleCreate, create_role),
    (server_admin_contract::domain_types::AdminHtmlAction::RoleUpdate, update_role),
    (server_admin_contract::domain_types::AdminHtmlAction::RoleDelete, delete_role),
    (server_admin_contract::domain_types::AdminHtmlAction::RolePermissions, role_permissions),
)]
struct AdminHtmlRoleActionRouteRegistry;
