#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
pub(super) enum AdminCrudPage {
    RoleCreate,
    RoleManage,
    UserCreate,
    UserManage,
}
