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

impl From<crate::admin_permission_id::AdminPermissionId> for AdminRoutePath {
    fn from(value: crate::admin_permission_id::AdminPermissionId) -> Self {
        Self(
            format!(
                "{}/{}",
                crate::admin_frontend_path::AdminFrontendPath::Permissions.get(),
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

impl From<crate::admin_role_permission_id::AdminRolePermissionId> for AdminRoutePath {
    fn from(value: crate::admin_role_permission_id::AdminRolePermissionId) -> Self {
        Self(
            format!(
                "{}/{}",
                crate::admin_data_table::AdminDataTable::RolePermissions.frontend_path(),
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
