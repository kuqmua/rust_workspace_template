#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) enum AdminRecordReadPath {
    AuditLog(crate::admin_audit_log_id::AdminAuditLogId),
    CleanupStatus(crate::admin_cleanup_status_id::AdminCleanupStatusId),
    LoginAttempt(crate::admin_login_attempt_id::AdminLoginAttemptId),
    PermissionAction(crate::admin_permission_action_id::AdminPermissionActionId),
    PermissionResourceAction(
        crate::admin_permission_resource_action_id::AdminPermissionResourceActionId,
    ),
    PermissionResource(crate::admin_permission_resource_id::AdminPermissionResourceId),
    RateLimit(crate::admin_rate_limit_id::AdminRateLimitId),
    Role(crate::admin_role_id::AdminRoleId),
    RoleRule(crate::admin_role_rule_id::AdminRoleRuleId),
    Rule(crate::admin_rule_id::AdminRuleId),
    SystemSetting(crate::admin_system_setting_id::AdminSystemSettingId),
    User(crate::admin_user_id::AdminUserId),
}
