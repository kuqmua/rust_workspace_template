#[derive(
    generate_constructor::New,
    optimal_memory_layout::OptimalMemoryLayout,
    generate_accessor::Getters,
)]
#[getters(get_mut)]
pub(crate) struct AdminAuditQueryParts {
    created_after: Option<server_admin_contract::admin_audit_timestamp::AdminAuditTimestamp>,
    created_before: Option<server_admin_contract::admin_audit_timestamp::AdminAuditTimestamp>,
    cursor_created_at: Option<server_admin_contract::admin_audit_timestamp::AdminAuditTimestamp>,
    cursor_id: Option<server_admin_contract::admin_audit_log_id::AdminAuditLogId>,
    resource_id: Option<server_admin_contract::admin_text::AdminText>,
    user_id: Option<server_admin_core::admin_user_record_id::AdminUserRecordId>,
    user_login: Option<server_admin_contract::admin_login::AdminLogin>,
    offset: server_admin_contract::admin_page_offset::AdminPageOffset,
    limit: server_admin_contract::admin_page_limit::AdminPageLimit,
    resource: Option<crate::admin_audit_resource::AdminAuditResource>,
    succeeded: Option<server_admin_contract::admin_bool::AdminBool>,
    action: Option<crate::admin_audit_action::AdminAuditAction>,
}
