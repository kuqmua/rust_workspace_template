#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    frontend_contract::domain_types::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new, into_parts)]
#[serde(deny_unknown_fields)]
pub struct AdminSetRolePermissionsReq {
    expected_permission_ids: crate::domain_types::AdminPermissionIds,
    permission_ids: crate::domain_types::AdminPermissionIds,
}
