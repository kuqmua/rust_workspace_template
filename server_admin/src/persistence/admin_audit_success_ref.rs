#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the application-layer audit event is constructed by sibling workflows and persisted only by this module"
)]

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy)]
pub(in super::super) struct AdminAuditSuccessRef<'value_lt> {
    pub(in super::super) action: super::super::super::AdminAuditAction,
    pub(in super::super) login: &'value_lt super::super::super::AdminLogin,
    pub(in super::super) resource: super::super::super::AdminAuditResource,
    pub(in super::super) resource_id: super::AdminAuditResourceId,
    pub(in super::super) user_id: super::super::super::AdminUserId,
}
