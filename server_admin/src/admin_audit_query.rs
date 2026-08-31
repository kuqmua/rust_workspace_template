#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, serde::Deserialize, utoipa::IntoParams,
)]
#[into_params(parameter_in = Query)]
#[derive(generate_accessor::Getters)]
pub struct AdminAuditQuery {
    created_after: Option<server_admin_contract::admin_audit_timestamp::AdminAuditTimestamp>,
    created_before: Option<server_admin_contract::admin_audit_timestamp::AdminAuditTimestamp>,
    cursor_created_at: Option<server_admin_contract::admin_audit_timestamp::AdminAuditTimestamp>,
    cursor_id: Option<server_admin_contract::admin_audit_log_id::AdminAuditLogId>,
    resource_id: Option<server_admin_contract::admin_text::AdminText>,
    #[param(inline)]
    user_id: Option<server_admin_core::admin_user_record_id::AdminUserRecordId>,
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
        crate::admin_audit_query_parts::AdminAuditQueryParts::new(
            self.created_after,
            self.created_before,
            self.cursor_created_at,
            self.cursor_id,
            self.resource_id,
            self.user_id,
            self.user_login,
            self.offset,
            self.limit,
            self.resource,
            self.succeeded,
            self.action,
        )
    }
}
