#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[allow(clippy::field_scoped_visibility_modifiers)] // repository query binding consumes this internal cross-module DTO field-by-field
#[derive(generate_accessor::Getters)]
#[getters(get_mut)]
pub(crate) struct AdminAuditQueryParts {
    created_after: Option<server_admin_contract::domain_types::AdminAuditTimestamp>,
    created_before: Option<server_admin_contract::domain_types::AdminAuditTimestamp>,
    cursor_created_at: Option<server_admin_contract::domain_types::AdminAuditTimestamp>,
    cursor_id: Option<server_admin_contract::domain_types::AdminAuditLogId>,
    resource_id: Option<server_admin_contract::domain_types::AdminText>,
    user_id: Option<crate::AdminUserId>,
    user_login: Option<server_admin_contract::domain_types::AdminLogin>,
    offset: server_admin_contract::domain_types::AdminPageOffset,
    limit: server_admin_contract::domain_types::AdminPageLimit,
    resource: Option<crate::AdminAuditResource>,
    succeeded: Option<server_admin_contract::domain_types::AdminBool>,
    action: Option<crate::AdminAuditAction>,
}
#[allow(
    dead_code,
    reason = "field access is intentionally encapsulated behind uniform getters"
)]
impl AdminAuditQueryParts {
    #[allow(
        clippy::too_many_arguments,
        reason = "constructor keeps private audit query fields inside the domain type"
    )]
    pub(crate) const fn new(
        action: Option<crate::AdminAuditAction>,
        created_after: Option<server_admin_contract::domain_types::AdminAuditTimestamp>,
        created_before: Option<server_admin_contract::domain_types::AdminAuditTimestamp>,
        cursor_created_at: Option<server_admin_contract::domain_types::AdminAuditTimestamp>,
        cursor_id: Option<server_admin_contract::domain_types::AdminAuditLogId>,
        limit: server_admin_contract::domain_types::AdminPageLimit,
        offset: server_admin_contract::domain_types::AdminPageOffset,
        resource: Option<crate::AdminAuditResource>,
        resource_id: Option<server_admin_contract::domain_types::AdminText>,
        succeeded: Option<server_admin_contract::domain_types::AdminBool>,
        user_id: Option<crate::AdminUserId>,
        user_login: Option<server_admin_contract::domain_types::AdminLogin>,
    ) -> Self {
        Self {
            created_after,
            created_before,
            cursor_created_at,
            cursor_id,
            resource_id,
            user_id,
            user_login,
            offset,
            limit,
            resource,
            succeeded,
            action,
        }
    }
}
