#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, serde::Deserialize, utoipa::IntoParams,
)]
#[into_params(parameter_in = Query)]
pub struct AdminAuditQuery {
    created_after: Option<server_admin_contract::admin_audit_timestamp::AdminAuditTimestamp>,
    created_before: Option<server_admin_contract::admin_audit_timestamp::AdminAuditTimestamp>,
    cursor_created_at: Option<server_admin_contract::admin_audit_timestamp::AdminAuditTimestamp>,
    cursor_id: Option<server_admin_contract::admin_audit_log_id::AdminAuditLogId>,
    resource_id: Option<server_admin_contract::admin_text::AdminText>,
    #[param(inline)]
    user_id: Option<server_admin_core::admin_user_id::AdminUserId>,
    user_login: Option<server_admin_contract::admin_login::AdminLogin>,
    #[serde(default)]
    #[param(value_type = u32)]
    offset: server_admin_contract::admin_page_offset::AdminPageOffset,
    #[serde(default)]
    #[param(value_type = u16, minimum = 1, maximum = 100)]
    limit: server_admin_contract::admin_page_limit::AdminPageLimit,
    #[param(inline)]
    resource: Option<crate::admin_audit_resource::AdminAuditResource>,
    succeeded: Option<server_admin_contract::admin_bool::AdminBool>,
    #[param(inline)]
    action: Option<crate::admin_audit_action::AdminAuditAction>,
}
impl AdminAuditQuery {
    pub(crate) fn cursor_is_complete(&self) -> server_admin_core::std_admin_bool::StdAdminBool {
        server_admin_core::std_admin_bool::StdAdminBool::from(
            self.cursor_created_at.is_some() == self.cursor_id.is_some(),
        )
    }
    pub(crate) fn into_parts(self) -> crate::admin_audit_query_parts::AdminAuditQueryParts {
        crate::admin_audit_query_parts::AdminAuditQueryParts {
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
