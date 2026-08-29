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
