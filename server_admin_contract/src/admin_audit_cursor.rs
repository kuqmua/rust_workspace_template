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
pub struct AdminAuditCursor {
    #[contract_struct_api(borrow)]
    created_at: crate::domain_types::AdminAuditTimestamp,
    #[contract_struct_api(copy_ref)]
    id: crate::domain_types::AdminAuditLogId,
}
