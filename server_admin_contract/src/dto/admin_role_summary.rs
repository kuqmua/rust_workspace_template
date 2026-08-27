#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    frontend_contract::domain_types::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new)]
#[optimal_memory_layout(skip)]
pub struct AdminRoleSummary {
    #[contract_struct_api(copy_ref)]
    id: crate::domain_types::AdminRoleId,
    #[contract_struct_api(copy_ref)]
    is_system: crate::domain_types::AdminBool,
    name: crate::domain_types::AdminRoleName,
    #[serde(default)]
    #[contract_struct_api(slice = crate::domain_types::AdminPermissionId)]
    permission_ids: crate::domain_types::AdminPermissionIds,
}
impl AdminRoleSummary {
    #[must_use]
    #[allow(clippy::same_name_method)] // Utoipa 5's static schema name intentionally coexists with this domain accessor
    pub const fn name(&self) -> &crate::domain_types::AdminRoleName {
        &self.name
    }
}
