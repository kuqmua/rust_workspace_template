#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, PartialEq, Eq,
)]
pub enum AdminCookieKind {
    Access,
    Csrf,
    Refresh,
}

impl AdminCookieKind {
    pub(crate) fn name(self) -> server_admin_core::std_admin_str_ref::StdAdminStrRef<'static> {
        server_admin_core::std_admin_str_ref::StdAdminStrRef::from(match self {
            Self::Access => constants_str::SERVER_ADMIN_ACCESS_COOKIE_NAME,
            Self::Csrf => constants_str::ADMIN_CSRF_TOKEN,
            Self::Refresh => constants_str::ADMIN_REFRESH_TOKEN,
        })
    }
}
