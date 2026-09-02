#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
pub(crate) enum AdminCrudPage {
    RoleCreate,
    RoleManage,
    UserCreate,
    UserManage,
}
