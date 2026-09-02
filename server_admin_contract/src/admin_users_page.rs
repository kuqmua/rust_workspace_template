#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    proc_macro_frontend_contract::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new)]
pub struct AdminUsersPage {
    #[contract_struct_api(into, slice = crate::admin_user_summary::AdminUserSummary)]
    items: crate::admin_user_summaries::AdminUserSummaries,
    #[contract_struct_api(slice = crate::admin_role_summary::AdminRoleSummary)]
    roles: crate::admin_role_summaries::AdminRoleSummaries,
    #[schema(value_type = u64)]
    #[contract_struct_api(copy_ref)]
    total: crate::admin_page_total::AdminPageTotal,
}
