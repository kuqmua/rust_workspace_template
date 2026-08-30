#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub(crate) enum AssignmentFormTarget {
    RolePermissions(crate::role_permissions_form::RolePermissionsForm),
    UserRoles(crate::user_roles_form::UserRolesForm),
}
