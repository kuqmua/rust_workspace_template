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
pub struct AdminChangeOwnPasswordReq {
    current_password: crate::admin_password::AdminPassword,
    new_password: crate::admin_new_password::AdminNewPassword,
}
