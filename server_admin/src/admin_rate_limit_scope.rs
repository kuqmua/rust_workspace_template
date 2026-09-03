#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_naming_enum_with_unit_fields_to_snake_case_str::EnumWithUnitFieldsToSnakeCaseStr,
)]
pub(crate) enum AdminRateLimitScope {
    AuditExport,
    Mutation,
    RefreshIp,
    SignInIp,
    SignInIpLogin,
}

impl AdminRateLimitScope {
    #[allow(
        clippy::single_call_fn,
        reason = "admin rate limit scope remains a named owner because its boundary role is clearer and directly testable"
    )]
    pub(crate) fn as_str(self) -> server_admin_core::std_admin_str_ref::StdAdminStrRef<'static> {
        server_admin_core::std_admin_str_ref::StdAdminStrRef::from(self.as_snake_case_str())
    }
}
