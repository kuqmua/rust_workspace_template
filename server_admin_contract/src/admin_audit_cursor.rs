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
pub struct AdminAuditCursor {
    #[contract_struct_api(borrow)]
    created_at: crate::admin_audit_timestamp::AdminAuditTimestamp,
    #[contract_struct_api(copy_ref)]
    id: crate::admin_audit_log_id::AdminAuditLogId,
}
