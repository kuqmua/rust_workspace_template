#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_naming::EnumWithUnitFieldsToSnakeCaseStr,
)]
pub(crate) enum AdminRateLimitScope {
    AuditExport,
    Mutation,
    RefreshIp,
    SignInIp,
    SignInIpLogin,
}

impl AdminRateLimitScope {
    #[allow(clippy::single_call_fn, reason = "lint suppression is required here")]
    pub(crate) fn as_str(self) -> server_admin_core::std_admin_str_ref::StdAdminStrRef<'static> {
        server_admin_core::std_admin_str_ref::StdAdminStrRef::from(self.as_snake_case_str())
    }
}
