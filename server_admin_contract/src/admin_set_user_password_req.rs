#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    frontend_contract_macros::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new)]
#[serde(deny_unknown_fields)]
pub struct AdminSetUserPasswordReq {
    #[contract_struct_api(into)]
    password: crate::admin_new_password::AdminNewPassword,
}
