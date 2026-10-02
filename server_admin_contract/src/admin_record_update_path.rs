#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) enum AdminRecordUpdatePath {
    User(crate::admin_user_update_path::AdminUserUpdatePath),
    RolePage(crate::admin_role_id::AdminRoleId),
    RoleAction(crate::admin_role_id::AdminRoleId),
}
