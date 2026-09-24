#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    proc_macro_newtype_as_ref_str::AsRefStr,
    proc_macro_newtype_display::Display,
)]
pub struct AdminRoutePath(Box<str>);
impl TryFrom<String> for AdminRoutePath {
    type Error = crate::admin_route_path_error::AdminRoutePathError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > constants_usize::VALUE_8_192 {
            Err(crate::admin_route_path_error::AdminRoutePathError::TooLong)
        } else {
            Ok(Self(value.into_boxed_str()))
        }
    }
}

impl From<crate::admin_access_session_id::AdminAccessSessionId> for AdminRoutePath {
    fn from(value: crate::admin_access_session_id::AdminAccessSessionId) -> Self {
        Self(
            format!(
                "{}/{}",
                crate::admin_data_table::AdminDataTable::AccessSessions.frontend_path(),
                value
            )
            .into_boxed_str(),
        )
    }
}

impl From<crate::admin_audit_log_id::AdminAuditLogId> for AdminRoutePath {
    fn from(value: crate::admin_audit_log_id::AdminAuditLogId) -> Self {
        Self(
            format!(
                "{}/{}",
                crate::admin_data_table::AdminDataTable::AuditLog.frontend_path(),
                value
            )
            .into_boxed_str(),
        )
    }
}

impl From<crate::admin_cleanup_status_id::AdminCleanupStatusId> for AdminRoutePath {
    fn from(value: crate::admin_cleanup_status_id::AdminCleanupStatusId) -> Self {
        Self(
            format!(
                "{}/{}",
                crate::admin_data_table::AdminDataTable::CleanupStatus.frontend_path(),
                value
            )
            .into_boxed_str(),
        )
    }
}

impl From<crate::admin_login_attempt_id::AdminLoginAttemptId> for AdminRoutePath {
    fn from(value: crate::admin_login_attempt_id::AdminLoginAttemptId) -> Self {
        Self(
            format!(
                "{}/{}",
                crate::admin_data_table::AdminDataTable::LoginAttempts.frontend_path(),
                value
            )
            .into_boxed_str(),
        )
    }
}

impl From<crate::admin_rate_limit_id::AdminRateLimitId> for AdminRoutePath {
    fn from(value: crate::admin_rate_limit_id::AdminRateLimitId) -> Self {
        Self(
            format!(
                "{}/{}",
                crate::admin_data_table::AdminDataTable::RateLimits.frontend_path(),
                value
            )
            .into_boxed_str(),
        )
    }
}

impl From<crate::admin_user_id::AdminUserId> for AdminRoutePath {
    fn from(value: crate::admin_user_id::AdminUserId) -> Self {
        Self(
            format!(
                "{}/{}",
                crate::admin_frontend_path::AdminFrontendPath::Users.get(),
                value
            )
            .into_boxed_str(),
        )
    }
}

impl From<crate::admin_permission_resource_id::AdminPermissionResourceId> for AdminRoutePath {
    fn from(value: crate::admin_permission_resource_id::AdminPermissionResourceId) -> Self {
        Self(
            format!(
                "{}/{}",
                crate::admin_data_table::AdminDataTable::PermissionResources.frontend_path(),
                value
            )
            .into_boxed_str(),
        )
    }
}

impl From<crate::admin_permission_resource_action_id::AdminPermissionResourceActionId>
    for AdminRoutePath
{
    fn from(
        value: crate::admin_permission_resource_action_id::AdminPermissionResourceActionId,
    ) -> Self {
        Self(
            format!(
                "{}/{}",
                crate::admin_data_table::AdminDataTable::PermissionResourceActions.frontend_path(),
                value
            )
            .into_boxed_str(),
        )
    }
}

impl From<crate::admin_permission_action_id::AdminPermissionActionId> for AdminRoutePath {
    fn from(value: crate::admin_permission_action_id::AdminPermissionActionId) -> Self {
        Self(
            format!(
                "{}/{}",
                crate::admin_data_table::AdminDataTable::PermissionActions.frontend_path(),
                value
            )
            .into_boxed_str(),
        )
    }
}

impl From<crate::admin_role_id::AdminRoleId> for AdminRoutePath {
    fn from(value: crate::admin_role_id::AdminRoleId) -> Self {
        Self(
            format!(
                "{}/{}",
                crate::admin_frontend_path::AdminFrontendPath::Roles.get(),
                value
            )
            .into_boxed_str(),
        )
    }
}

impl From<crate::admin_rule_id::AdminRuleId> for AdminRoutePath {
    fn from(value: crate::admin_rule_id::AdminRuleId) -> Self {
        Self(
            format!(
                "{}/{}",
                crate::admin_frontend_path::AdminFrontendPath::Rules.get(),
                value
            )
            .into_boxed_str(),
        )
    }
}

impl From<crate::admin_user_role_id::AdminUserRoleId> for AdminRoutePath {
    fn from(value: crate::admin_user_role_id::AdminUserRoleId) -> Self {
        Self(
            format!(
                "{}/{}",
                crate::admin_data_table::AdminDataTable::UserRoles.frontend_path(),
                value
            )
            .into_boxed_str(),
        )
    }
}

impl From<crate::admin_role_rule_id::AdminRoleRuleId> for AdminRoutePath {
    fn from(value: crate::admin_role_rule_id::AdminRoleRuleId) -> Self {
        Self(
            format!(
                "{}/{}",
                crate::admin_data_table::AdminDataTable::RoleRules.frontend_path(),
                value
            )
            .into_boxed_str(),
        )
    }
}

impl From<crate::admin_refresh_token_id::AdminRefreshTokenId> for AdminRoutePath {
    fn from(value: crate::admin_refresh_token_id::AdminRefreshTokenId) -> Self {
        Self(
            format!(
                "{}/{}",
                crate::admin_data_table::AdminDataTable::RefreshTokens.frontend_path(),
                value
            )
            .into_boxed_str(),
        )
    }
}

impl From<crate::admin_system_setting_id::AdminSystemSettingId> for AdminRoutePath {
    fn from(value: crate::admin_system_setting_id::AdminSystemSettingId) -> Self {
        Self(
            format!(
                "{}/{}",
                crate::admin_data_table::AdminDataTable::SystemSettings.frontend_path(),
                value
            )
            .into_boxed_str(),
        )
    }
}
