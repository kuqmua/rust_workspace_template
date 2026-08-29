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
pub struct AdminCreateUserReq {
    display_name: crate::admin_display_name::AdminDisplayName,
    login: crate::admin_login::AdminLogin,
    password: crate::admin_new_password::AdminNewPassword,
}
