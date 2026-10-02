#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
pub(crate) enum AdminCrudPage {
    RoleCreate,
    RoleManage,
    RoleUpdate(Option<server_admin_contract::admin_role_id::AdminRoleId>),
    UserCreate,
    UserManage,
    UserUpdate(server_admin_contract::admin_user_id::AdminUserId),
}
