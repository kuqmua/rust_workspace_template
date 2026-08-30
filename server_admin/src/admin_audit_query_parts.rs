#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[allow(clippy::field_scoped_visibility_modifiers)] // repository query binding consumes this internal cross-module DTO field-by-field
#[derive(generate_accessor::Getters)]
#[getters(get_mut)]
pub(crate) struct AdminAuditQueryParts {
    pub(crate) created_after:
        Option<server_admin_contract::admin_audit_timestamp::AdminAuditTimestamp>,
    pub(crate) created_before:
        Option<server_admin_contract::admin_audit_timestamp::AdminAuditTimestamp>,
    pub(crate) cursor_created_at:
        Option<server_admin_contract::admin_audit_timestamp::AdminAuditTimestamp>,
    pub(crate) cursor_id: Option<server_admin_contract::admin_audit_log_id::AdminAuditLogId>,
    pub(crate) resource_id: Option<server_admin_contract::admin_text::AdminText>,
    pub(crate) user_id: Option<server_admin_core::admin_user_record_id::AdminUserRecordId>,
    pub(crate) user_login: Option<server_admin_contract::admin_login::AdminLogin>,
    pub(crate) offset: server_admin_contract::admin_page_offset::AdminPageOffset,
    pub(crate) limit: server_admin_contract::admin_page_limit::AdminPageLimit,
    pub(crate) resource: Option<crate::admin_audit_resource::AdminAuditResource>,
    pub(crate) succeeded: Option<server_admin_contract::admin_bool::AdminBool>,
    pub(crate) action: Option<crate::admin_audit_action::AdminAuditAction>,
}
