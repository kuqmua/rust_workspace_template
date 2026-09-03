#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    proc_macro_frontend_contract_derive_contract_struct_api::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new)]
#[optimal_memory_layout(skip)]
pub struct AdminUserSummary {
    #[contract_struct_api(borrow)]
    display_name: crate::admin_display_name::AdminDisplayName,
    #[contract_struct_api(copy_ref)]
    id: crate::admin_user_id::AdminUserId,
    #[contract_struct_api(copy_ref)]
    is_banned: crate::admin_bool::AdminBool,
    #[contract_struct_api(borrow)]
    login: crate::admin_login::AdminLogin,
    #[serde(default)]
    #[contract_struct_api(slice = crate::admin_role_id::AdminRoleId)]
    role_ids: crate::admin_role_ids::AdminRoleIds,
}
