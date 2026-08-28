#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    frontend_contract::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new)]
#[serde(deny_unknown_fields)]
pub struct AdminCreateRoleReq {
    #[contract_struct_api(into)]
    name: crate::domain_types::AdminRoleName,
}
