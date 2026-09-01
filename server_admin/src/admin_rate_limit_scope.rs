#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    naming_macros::EnumWithUnitFieldsToSnakeCaseStr,
)]
pub(crate) enum AdminRateLimitScope {
    AuditExport,
    Mutation,
    RefreshIp,
    SignInIp,
    SignInIpLogin,
}

impl AdminRateLimitScope {
    #[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
    pub(crate) fn as_str(self) -> server_admin_core::std_admin_str_ref::StdAdminStrRef<'static> {
        server_admin_core::std_admin_str_ref::StdAdminStrRef::from(self.as_snake_case_str())
    }
}
