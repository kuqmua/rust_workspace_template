#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the application-layer audit event is constructed by sibling workflows and persisted only by this module"
)]

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy)]
pub(crate) struct AdminAuditSuccessRef<'value_lt> {
    pub(crate) action: crate::AdminAuditAction,
    pub(crate) login: &'value_lt crate::AdminLogin,
    pub(crate) resource: crate::AdminAuditResource,
    pub(crate) resource_id: crate::AdminAuditResourceId,
    pub(crate) user_id: crate::AdminUserId,
}
