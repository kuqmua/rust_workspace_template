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
#[getters(bare)]
#[contract_struct_api(new, into_parts)]
#[serde(deny_unknown_fields)]
pub struct AdminUpdateUserRequest {
    display_name: Option<crate::admin_display_name::AdminDisplayName>,
    login: Option<crate::admin_login::AdminLogin>,
    #[schema(write_only)]
    password: Option<crate::admin_new_password::AdminNewPassword>,
    expected_role_ids: Option<crate::admin_role_ids::AdminRoleIds>,
    role_ids: Option<crate::admin_role_ids::AdminRoleIds>,
    is_banned: Option<crate::admin_bool::AdminBool>,
}
