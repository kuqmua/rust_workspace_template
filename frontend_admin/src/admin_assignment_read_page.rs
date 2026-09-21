#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq,
)]
pub(crate) enum AdminAssignmentReadPage {
    RolePermission,
    UserRole,
}
impl AdminAssignmentReadPage {
    pub(crate) const fn data_page(self) -> &'static str {
        match self {
            Self::RolePermission => constants_str::ADMIN_ROLE_PERMISSION_READ_PAGE,
            Self::UserRole => constants_str::ADMIN_USER_ROLE_READ_PAGE,
        }
    }
}
