#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    frontend_contract::domain_types::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new)]
#[optimal_memory_layout(skip)]
pub struct AdminAuditView {
    #[contract_struct_api(borrow)]
    action: crate::domain_types::AdminText,
    #[contract_struct_api(borrow)]
    created_at: crate::domain_types::AdminAuditTimestamp,
    #[contract_struct_api(option_borrow)]
    details: Option<crate::domain_types::SerdeJsonAdminAuditDetails>,
    #[contract_struct_api(copy_ref)]
    id: crate::domain_types::AdminAuditLogId,
    #[contract_struct_api(borrow)]
    resource: crate::domain_types::AdminText,
    #[contract_struct_api(option_borrow)]
    resource_id: Option<crate::domain_types::AdminText>,
    #[contract_struct_api(copy_ref)]
    succeeded: crate::domain_types::AdminBool,
    #[contract_struct_api(copy_ref)]
    user_id: Option<crate::domain_types::AdminUserId>,
    #[contract_struct_api(option_borrow)]
    user_login: Option<crate::domain_types::AdminLogin>,
}
