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
pub struct AdminAuditPage {
    #[contract_struct_api(slice = crate::admin_audit_view::AdminAuditView)]
    items: crate::admin_audit_views::AdminAuditViews,
    #[schema(inline)]
    #[contract_struct_api(option_borrow)]
    next_cursor: Option<crate::admin_audit_cursor::AdminAuditCursor>,
    #[schema(value_type = u64)]
    #[contract_struct_api(copy_ref)]
    total: crate::admin_page_total::AdminPageTotal,
}
