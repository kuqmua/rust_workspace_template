#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the application-layer audit event is constructed by sibling workflows and persisted only by this module"
)]

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy)]
pub(crate) struct AdminAuditSuccessRef<'value_lt> {
    pub(crate) action: crate::admin_audit_action::AdminAuditAction,
    pub(crate) login: &'value_lt server_admin_contract::admin_login::AdminLogin,
    pub(crate) resource: crate::admin_audit_resource::AdminAuditResource,
    pub(crate) resource_id: crate::admin_audit_resource_id::AdminAuditResourceId,
    pub(crate) user_id: server_admin_core::admin_user_record_id::AdminUserRecordId,
}
