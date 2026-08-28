use super::{AdminAuditCursor, AdminAuditView};

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    frontend_contract::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new)]
pub struct AdminAuditPage {
    #[contract_struct_api(slice = AdminAuditView)]
    items: crate::domain_types::AdminAuditViews,
    #[schema(inline)]
    #[contract_struct_api(option_borrow)]
    next_cursor: Option<AdminAuditCursor>,
    #[schema(value_type = u64)]
    #[contract_struct_api(copy_ref)]
    total: crate::domain_types::AdminPageTotal,
}
