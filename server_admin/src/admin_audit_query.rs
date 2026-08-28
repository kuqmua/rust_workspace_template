use crate::AdminAuditQueryParts;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, serde::Deserialize, utoipa::IntoParams,
)]
#[into_params(parameter_in = Query)]
pub struct AdminAuditQuery {
    created_after: Option<server_admin_contract::domain_types::AdminAuditTimestamp>,
    created_before: Option<server_admin_contract::domain_types::AdminAuditTimestamp>,
    cursor_created_at: Option<server_admin_contract::domain_types::AdminAuditTimestamp>,
    cursor_id: Option<server_admin_contract::domain_types::AdminAuditLogId>,
    resource_id: Option<server_admin_contract::domain_types::AdminText>,
    #[param(inline)]
    user_id: Option<crate::AdminUserId>,
    user_login: Option<server_admin_contract::domain_types::AdminLogin>,
    #[serde(default)]
    #[param(value_type = u32)]
    offset: server_admin_contract::domain_types::AdminPageOffset,
    #[serde(default)]
    #[param(value_type = u16, minimum = 1, maximum = 100)]
    limit: server_admin_contract::domain_types::AdminPageLimit,
    #[param(inline)]
    resource: Option<crate::AdminAuditResource>,
    succeeded: Option<server_admin_contract::domain_types::AdminBool>,
    #[param(inline)]
    action: Option<crate::AdminAuditAction>,
}
impl AdminAuditQuery {
    pub(crate) fn cursor_is_complete(&self) -> crate::StdAdminBool {
        crate::StdAdminBool::from(self.cursor_created_at.is_some() == self.cursor_id.is_some())
    }
    pub(crate) fn into_parts(self) -> AdminAuditQueryParts {
        AdminAuditQueryParts {
            action: self.action,
            created_after: self.created_after,
            created_before: self.created_before,
            cursor_created_at: self.cursor_created_at,
            cursor_id: self.cursor_id,
            limit: self.limit,
            offset: self.offset,
            resource: self.resource,
            resource_id: self.resource_id,
            succeeded: self.succeeded,
            user_id: self.user_id,
            user_login: self.user_login,
        }
    }
}
