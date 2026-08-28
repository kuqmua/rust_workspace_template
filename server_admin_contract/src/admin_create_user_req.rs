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
pub struct AdminCreateUserReq {
    display_name: crate::domain_types::AdminDisplayName,
    login: crate::domain_types::AdminLogin,
    password: crate::domain_types::AdminNewPassword,
}
