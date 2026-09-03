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
pub struct AdminAuditView {
    #[contract_struct_api(borrow)]
    action: crate::admin_text::AdminText,
    #[contract_struct_api(borrow)]
    created_at: crate::admin_audit_timestamp::AdminAuditTimestamp,
    #[contract_struct_api(option_borrow)]
    details: Option<crate::serde_json_admin_audit_details::SerdeJsonAdminAuditDetails>,
    #[contract_struct_api(copy_ref)]
    id: crate::admin_audit_log_id::AdminAuditLogId,
    #[contract_struct_api(borrow)]
    resource: crate::admin_text::AdminText,
    #[contract_struct_api(option_borrow)]
    resource_id: Option<crate::admin_text::AdminText>,
    #[contract_struct_api(copy_ref)]
    succeeded: crate::admin_bool::AdminBool,
    #[contract_struct_api(copy_ref)]
    user_id: Option<crate::admin_user_id::AdminUserId>,
    #[contract_struct_api(option_borrow)]
    user_login: Option<crate::admin_login::AdminLogin>,
}
