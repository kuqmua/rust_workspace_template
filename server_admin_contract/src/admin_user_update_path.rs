#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug)]
pub enum AdminUserUpdatePath {
    Page(crate::admin_user_id::AdminUserId),
    Action(crate::admin_user_id::AdminUserId),
}
