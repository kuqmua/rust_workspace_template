#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
pub(crate) enum AdminCrudPage {
    RoleCreate,
    RoleManage,
    RoleUpdate,
    UserCreate,
    UserManage,
    UserUpdate(server_admin_contract::admin_user_id::AdminUserId),
}
