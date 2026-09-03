#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    proc_macro_frontend_contract_derive_contract_struct_api::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new, into_parts)]
#[serde(deny_unknown_fields)]
pub struct AdminSetRolePermissionsRequest {
    expected_permission_ids: crate::admin_permission_ids::AdminPermissionIds,
    permission_ids: crate::admin_permission_ids::AdminPermissionIds,
}
