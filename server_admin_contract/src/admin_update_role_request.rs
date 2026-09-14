#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_getters::Getters,
    Clone,
    Debug,
    proc_macro_frontend_contract_derive_contract_struct_api::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new)]
#[serde(deny_unknown_fields)]
pub struct AdminUpdateRoleRequest {
    #[contract_struct_api(into)]
    name: Option<crate::admin_role_name::AdminRoleName>,
    permissions: Option<crate::admin_set_role_permissions_request::AdminSetRolePermissionsRequest>,
}
