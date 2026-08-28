#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    frontend_contract::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new, into_parts)]
#[serde(deny_unknown_fields)]
pub struct AdminSetUserRolesReq {
    expected_role_ids: crate::domain_types::AdminRoleIds,
    role_ids: crate::domain_types::AdminRoleIds,
}
