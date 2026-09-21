#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq,
)]
pub(crate) enum AdminRecordReadPage {
    AccessSession,
    RefreshToken,
    RolePermission,
    UserRole,
}
impl AdminRecordReadPage {
    pub(crate) const fn data_page(self) -> &'static str {
        match self {
            Self::AccessSession => constants_str::ADMIN_ACCESS_SESSION_READ_PAGE,
            Self::RefreshToken => constants_str::ADMIN_REFRESH_TOKEN_READ_PAGE,
            Self::RolePermission => constants_str::ADMIN_ROLE_PERMISSION_READ_PAGE,
            Self::UserRole => constants_str::ADMIN_USER_ROLE_READ_PAGE,
        }
    }
}
