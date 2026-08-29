#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    frontend_contract_macros::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new, into_parts)]
#[serde(deny_unknown_fields)]
pub struct AdminSetRolePermissionsReq {
    expected_permission_ids: crate::admin_permission_ids::AdminPermissionIds,
    permission_ids: crate::admin_permission_ids::AdminPermissionIds,
}
