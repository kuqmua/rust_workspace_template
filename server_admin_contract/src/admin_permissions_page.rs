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
pub struct AdminPermissionsPage {
    #[contract_struct_api(into, slice = crate::admin_permission_summary::AdminPermissionSummary)]
    items: crate::admin_permission_summaries::AdminPermissionSummaries,
    #[schema(value_type = u64)]
    #[contract_struct_api(copy_ref)]
    total: crate::admin_page_total::AdminPageTotal,
}
