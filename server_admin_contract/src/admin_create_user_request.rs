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
pub struct AdminCreateUserRequest {
    display_name: crate::admin_display_name::AdminDisplayName,
    login: crate::admin_login::AdminLogin,
    password: crate::admin_new_password::AdminNewPassword,
}
