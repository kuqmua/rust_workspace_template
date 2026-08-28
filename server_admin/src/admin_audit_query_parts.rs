#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[allow(clippy::field_scoped_visibility_modifiers)] // repository query binding consumes this internal cross-module DTO field-by-field
#[derive(generate_accessor::Getters)]
#[getters(get_mut)]
pub(crate) struct AdminAuditQueryParts {
    pub(crate) created_after: Option<server_admin_contract::domain_types::AdminAuditTimestamp>,
    pub(crate) created_before: Option<server_admin_contract::domain_types::AdminAuditTimestamp>,
    pub(crate) cursor_created_at: Option<server_admin_contract::domain_types::AdminAuditTimestamp>,
    pub(crate) cursor_id: Option<server_admin_contract::domain_types::AdminAuditLogId>,
    pub(crate) resource_id: Option<server_admin_contract::domain_types::AdminText>,
    pub(crate) user_id: Option<crate::AdminUserId>,
    pub(crate) user_login: Option<server_admin_contract::domain_types::AdminLogin>,
    pub(crate) offset: server_admin_contract::domain_types::AdminPageOffset,
    pub(crate) limit: server_admin_contract::domain_types::AdminPageLimit,
    pub(crate) resource: Option<crate::AdminAuditResource>,
    pub(crate) succeeded: Option<server_admin_contract::domain_types::AdminBool>,
    pub(crate) action: Option<crate::AdminAuditAction>,
}
