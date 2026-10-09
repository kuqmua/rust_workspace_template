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

impl From<crate::admin_session_identifier::AdminSessionIdentifier> for AdminRoutePath {
    fn from(value: crate::admin_session_identifier::AdminSessionIdentifier) -> Self {
        Self::from(crate::admin_session_read_path::AdminSessionReadPath::Own(
            value,
        ))
    }
}

impl From<crate::admin_access_session_id::AdminAccessSessionId> for AdminRoutePath {
    fn from(value: crate::admin_access_session_id::AdminAccessSessionId) -> Self {
        Self::from(crate::admin_session_read_path::AdminSessionReadPath::Access(value))
    }
}

impl From<crate::admin_audit_log_id::AdminAuditLogId> for AdminRoutePath {
    fn from(value: crate::admin_audit_log_id::AdminAuditLogId) -> Self {
        Self::from(crate::admin_record_read_path::AdminRecordReadPath::AuditLog(value))
    }
}

impl From<crate::admin_cleanup_status_id::AdminCleanupStatusId> for AdminRoutePath {
    fn from(value: crate::admin_cleanup_status_id::AdminCleanupStatusId) -> Self {
        Self::from(crate::admin_record_read_path::AdminRecordReadPath::CleanupStatus(value))
    }
}

impl From<crate::admin_login_attempt_id::AdminLoginAttemptId> for AdminRoutePath {
    fn from(value: crate::admin_login_attempt_id::AdminLoginAttemptId) -> Self {
        Self::from(crate::admin_record_read_path::AdminRecordReadPath::LoginAttempt(value))
    }
}

impl From<crate::admin_rate_limit_id::AdminRateLimitId> for AdminRoutePath {
    fn from(value: crate::admin_rate_limit_id::AdminRateLimitId) -> Self {
        Self::from(crate::admin_record_read_path::AdminRecordReadPath::RateLimit(value))
    }
}

impl From<crate::admin_user_id::AdminUserId> for AdminRoutePath {
    fn from(value: crate::admin_user_id::AdminUserId) -> Self {
        Self::from(crate::admin_record_read_path::AdminRecordReadPath::User(
            value,
        ))
    }
}

impl From<crate::admin_permission_resource_id::AdminPermissionResourceId> for AdminRoutePath {
    fn from(value: crate::admin_permission_resource_id::AdminPermissionResourceId) -> Self {
        Self::from(crate::admin_record_read_path::AdminRecordReadPath::PermissionResource(value))
    }
}

impl From<crate::admin_permission_resource_action_id::AdminPermissionResourceActionId>
    for AdminRoutePath
{
    fn from(
        value: crate::admin_permission_resource_action_id::AdminPermissionResourceActionId,
    ) -> Self {
        Self::from(
            crate::admin_record_read_path::AdminRecordReadPath::PermissionResourceAction(value),
        )
    }
}

impl From<crate::admin_permission_action_id::AdminPermissionActionId> for AdminRoutePath {
    fn from(value: crate::admin_permission_action_id::AdminPermissionActionId) -> Self {
        Self::from(crate::admin_record_read_path::AdminRecordReadPath::PermissionAction(value))
    }
}

impl From<crate::admin_role_id::AdminRoleId> for AdminRoutePath {
    fn from(value: crate::admin_role_id::AdminRoleId) -> Self {
        Self::from(crate::admin_record_read_path::AdminRecordReadPath::Role(
            value,
        ))
    }
}

impl From<crate::admin_rule_id::AdminRuleId> for AdminRoutePath {
    fn from(value: crate::admin_rule_id::AdminRuleId) -> Self {
        Self::from(crate::admin_record_read_path::AdminRecordReadPath::Rule(
            value,
        ))
    }
}

impl From<crate::admin_user_role_id::AdminUserRoleId> for AdminRoutePath {
    fn from(value: crate::admin_user_role_id::AdminUserRoleId) -> Self {
        Self(
            crate::admin_frontend_path::AdminFrontendPath::UserRolesRead
                .get()
                .replace(
                    constants_str::ADMIN_USER_ROLE_ID_PLACEHOLDER,
                    &value.to_string(),
                )
                .into_boxed_str(),
        )
    }
}

impl From<crate::admin_role_rule_id::AdminRoleRuleId> for AdminRoutePath {
    fn from(value: crate::admin_role_rule_id::AdminRoleRuleId) -> Self {
        Self::from(crate::admin_record_read_path::AdminRecordReadPath::RoleRule(value))
    }
}

impl From<crate::admin_refresh_token_id::AdminRefreshTokenId> for AdminRoutePath {
    fn from(value: crate::admin_refresh_token_id::AdminRefreshTokenId) -> Self {
        Self::from(crate::admin_session_read_path::AdminSessionReadPath::Refresh(value))
    }
}

impl From<crate::admin_session_read_path::AdminSessionReadPath> for AdminRoutePath {
    fn from(value: crate::admin_session_read_path::AdminSessionReadPath) -> Self {
        let (admin_frontend_path, placeholder, identifier) = match &value {
            crate::admin_session_read_path::AdminSessionReadPath::Own(admin_session_identifier) => {
                (
                    crate::admin_frontend_path::AdminFrontendPath::SessionRead,
                    constants_str::ADMIN_SESSION_ID_PLACEHOLDER,
                    admin_session_identifier.to_string(),
                )
            }
            crate::admin_session_read_path::AdminSessionReadPath::Access(
                admin_access_session_id,
            ) => (
                crate::admin_frontend_path::AdminFrontendPath::AccessSessionsRead,
                constants_str::ADMIN_ACCESS_SESSION_ID_PLACEHOLDER,
                admin_access_session_id.to_string(),
            ),
            crate::admin_session_read_path::AdminSessionReadPath::Refresh(
                admin_refresh_token_id,
            ) => (
                crate::admin_frontend_path::AdminFrontendPath::RefreshTokensRead,
                constants_str::ADMIN_REFRESH_TOKEN_ID_PLACEHOLDER,
                admin_refresh_token_id.to_string(),
            ),
        };
        Self(
            admin_frontend_path
                .get()
                .replace(placeholder, &identifier)
                .into_boxed_str(),
        )
    }
}

impl From<crate::admin_system_setting_id::AdminSystemSettingId> for AdminRoutePath {
    fn from(value: crate::admin_system_setting_id::AdminSystemSettingId) -> Self {
        Self::from(crate::admin_record_read_path::AdminRecordReadPath::SystemSetting(value))
    }
}

impl From<crate::admin_user_update_path::AdminUserUpdatePath> for AdminRoutePath {
    fn from(value: crate::admin_user_update_path::AdminUserUpdatePath) -> Self {
        Self::from(crate::admin_record_update_path::AdminRecordUpdatePath::User(value))
    }
}
impl From<crate::admin_record_update_path::AdminRecordUpdatePath> for AdminRoutePath {
    fn from(value: crate::admin_record_update_path::AdminRecordUpdatePath) -> Self {
        let (template, placeholder, positive_non_zero_i64) = match value {
            crate::admin_record_update_path::AdminRecordUpdatePath::User(
                crate::admin_user_update_path::AdminUserUpdatePath::Page(admin_user_id),
            ) => (
                crate::admin_frontend_path::AdminFrontendPath::UsersUpdate.get(),
                constants_str::ADMIN_USER_ID_PLACEHOLDER,
                admin_user_id.value(),
            ),
            crate::admin_record_update_path::AdminRecordUpdatePath::User(
                crate::admin_user_update_path::AdminUserUpdatePath::Action(admin_user_id),
            ) => (
                crate::admin_html_action::AdminHtmlAction::UserRecordUpdate.get(),
                constants_str::ADMIN_USER_ID_PLACEHOLDER,
                admin_user_id.value(),
            ),
            crate::admin_record_update_path::AdminRecordUpdatePath::RolePage(admin_role_id) => (
                crate::admin_frontend_path::AdminFrontendPath::RoleRecordUpdate.get(),
                constants_str::ADMIN_ROLE_ID_PLACEHOLDER,
                admin_role_id.value(),
            ),
            crate::admin_record_update_path::AdminRecordUpdatePath::RoleAction(admin_role_id) => (
                crate::admin_html_action::AdminHtmlAction::RoleRecordUpdate.get(),
                constants_str::ADMIN_ROLE_ID_PLACEHOLDER,
                admin_role_id.value(),
            ),
        };
        Self(
            template
                .replace(placeholder, &positive_non_zero_i64.to_string())
                .into_boxed_str(),
        )
    }
}
impl From<crate::admin_record_read_path::AdminRecordReadPath> for AdminRoutePath {
    fn from(value: crate::admin_record_read_path::AdminRecordReadPath) -> Self {
        let (admin_frontend_path, placeholder, positive_non_zero_i64) = match value {
            crate::admin_record_read_path::AdminRecordReadPath::CleanupStatus(
                admin_cleanup_status_id,
            ) => (
                crate::admin_frontend_path::AdminFrontendPath::CleanupStatusesRead,
                constants_str::ADMIN_CLEANUP_STATUS_ID_PLACEHOLDER,
                admin_cleanup_status_id.value(),
            ),
            crate::admin_record_read_path::AdminRecordReadPath::RateLimit(admin_rate_limit_id) => (
                crate::admin_frontend_path::AdminFrontendPath::RateLimitsRead,
                constants_str::ADMIN_RATE_LIMIT_ID_PLACEHOLDER,
                admin_rate_limit_id.value(),
            ),
            crate::admin_record_read_path::AdminRecordReadPath::SystemSetting(
                admin_system_setting_id,
            ) => (
                crate::admin_frontend_path::AdminFrontendPath::SystemSettingsRead,
                constants_str::ADMIN_SYSTEM_SETTING_ID_PLACEHOLDER,
                admin_system_setting_id.value(),
            ),
            crate::admin_record_read_path::AdminRecordReadPath::AuditLog(admin_audit_log_id) => (
                crate::admin_frontend_path::AdminFrontendPath::AuditLogsRead,
                constants_str::ADMIN_AUDIT_LOG_ID_PLACEHOLDER,
                admin_audit_log_id.value(),
            ),
            crate::admin_record_read_path::AdminRecordReadPath::LoginAttempt(
                admin_login_attempt_id,
            ) => (
                crate::admin_frontend_path::AdminFrontendPath::LoginAttemptsRead,
                constants_str::ADMIN_LOGIN_ATTEMPT_ID_PLACEHOLDER,
                admin_login_attempt_id.value(),
            ),
            crate::admin_record_read_path::AdminRecordReadPath::Role(admin_role_id) => (
                crate::admin_frontend_path::AdminFrontendPath::RolesRead,
                constants_str::ADMIN_ROLE_ID_PLACEHOLDER,
                admin_role_id.value(),
            ),
            crate::admin_record_read_path::AdminRecordReadPath::RoleRule(admin_role_rule_id) => (
                crate::admin_frontend_path::AdminFrontendPath::RoleRulesRead,
                constants_str::ADMIN_ROLE_RULE_ID_PLACEHOLDER,
                admin_role_rule_id.value(),
            ),
            crate::admin_record_read_path::AdminRecordReadPath::Rule(admin_rule_id) => (
                crate::admin_frontend_path::AdminFrontendPath::RuleRecordRead,
                constants_str::ADMIN_RULE_ID_PLACEHOLDER,
                admin_rule_id.value(),
            ),
            crate::admin_record_read_path::AdminRecordReadPath::PermissionAction(
                admin_permission_action_id,
            ) => (
                crate::admin_frontend_path::AdminFrontendPath::PermissionActionsRead,
                constants_str::ADMIN_PERMISSION_ACTION_ID_PLACEHOLDER,
                admin_permission_action_id.value(),
            ),
            crate::admin_record_read_path::AdminRecordReadPath::PermissionResourceAction(
                admin_permission_resource_action_id,
            ) => (
                crate::admin_frontend_path::AdminFrontendPath::PermissionResourceActionsRead,
                constants_str::ADMIN_PERMISSION_RESOURCE_ACTION_ID_PLACEHOLDER,
                admin_permission_resource_action_id.value(),
            ),
            crate::admin_record_read_path::AdminRecordReadPath::PermissionResource(
                admin_permission_resource_id,
            ) => (
                crate::admin_frontend_path::AdminFrontendPath::PermissionResourcesRead,
                constants_str::ADMIN_PERMISSION_RESOURCE_ID_PLACEHOLDER,
                admin_permission_resource_id.value(),
            ),
            crate::admin_record_read_path::AdminRecordReadPath::User(admin_user_id) => (
                crate::admin_frontend_path::AdminFrontendPath::UsersRead,
                constants_str::ADMIN_USER_ID_PLACEHOLDER,
                admin_user_id.value(),
            ),
        };
        Self(
            admin_frontend_path
                .get()
                .replace(placeholder, &positive_non_zero_i64.to_string())
                .into_boxed_str(),
        )
    }
}
impl AdminRoutePath {
    #[must_use]
    pub fn role_update_path(admin_role_id: crate::admin_role_id::AdminRoleId) -> Self {
        Self::from(crate::admin_record_update_path::AdminRecordUpdatePath::RolePage(admin_role_id))
    }
    #[must_use]
    pub fn role_update_action_path(admin_role_id: crate::admin_role_id::AdminRoleId) -> Self {
        Self::from(
            crate::admin_record_update_path::AdminRecordUpdatePath::RoleAction(admin_role_id),
        )
    }

    #[must_use]
    pub fn user_update_path(admin_user_id: crate::admin_user_id::AdminUserId) -> Self {
        Self::from(crate::admin_user_update_path::AdminUserUpdatePath::Page(
            admin_user_id,
        ))
    }
    #[must_use]
    pub fn user_update_action_path(admin_user_id: crate::admin_user_id::AdminUserId) -> Self {
        Self::from(crate::admin_user_update_path::AdminUserUpdatePath::Action(
            admin_user_id,
        ))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_role_update_paths_use_the_selected_identifier() {
        let identifier = crate::admin_role_id::AdminRoleId::try_from(constants_i64::ONE);
        assert_eq!(identifier.iter().count(), constants_usize::ONE);
        if let Ok(admin_role_id) = identifier {
            let page = super::AdminRoutePath::role_update_path(admin_role_id);
            let action = super::AdminRoutePath::role_update_action_path(admin_role_id);
            assert_eq!(
                page.as_ref(),
                crate::admin_frontend_path::AdminFrontendPath::RoleRecordUpdate
                    .get()
                    .replace(
                        constants_str::ADMIN_ROLE_ID_PLACEHOLDER,
                        &admin_role_id.to_string()
                    )
            );
            assert_eq!(
                action.as_ref(),
                crate::admin_html_action::AdminHtmlAction::RoleRecordUpdate
                    .get()
                    .replace(
                        constants_str::ADMIN_ROLE_ID_PLACEHOLDER,
                        &admin_role_id.to_string()
                    )
            );
        }
    }

    #[test]
    fn test_admin_route_path_preserves_empty_and_ascii_byte_boundaries() {
        assert!(
            [0usize, 1usize, constants_usize::VALUE_8_192]
                .into_iter()
                .all(|length| {
                    super::AdminRoutePath::try_from(constants_str::SLASH.repeat(length)).is_ok_and(
                        |admin_route_path| {
                            admin_route_path.as_ref().len() == length
                                && admin_route_path.as_ref().bytes().all(|byte| byte == b'/')
                        },
                    )
                })
        );
        assert!(
            super::AdminRoutePath::try_from(
                constants_str::SLASH.repeat(constants_usize::VALUE_8_192 + 1usize)
            )
            .is_err_and(|error| matches!(
                error,
                crate::admin_route_path_error::AdminRoutePathError::TooLong
            ))
        );
    }

    #[test]
    fn test_admin_route_path_counts_utf8_bytes_and_preserves_null_text() {
        let character_count = 2_048usize;
        assert_eq!(
            character_count * char::MAX.len_utf8(),
            constants_usize::VALUE_8_192
        );
        assert!(
            super::AdminRoutePath::try_from(char::MAX.to_string().repeat(character_count))
                .is_ok_and(|admin_route_path| {
                    admin_route_path.as_ref().len() == constants_usize::VALUE_8_192
                        && admin_route_path.as_ref().chars().count() == character_count
                        && admin_route_path
                            .as_ref()
                            .chars()
                            .all(|character| character == char::MAX)
                })
        );
        assert!(
            super::AdminRoutePath::try_from(char::MAX.to_string().repeat(character_count + 1usize))
                .is_err_and(|error| matches!(
                    error,
                    crate::admin_route_path_error::AdminRoutePathError::TooLong
                ))
        );
        assert!(
            super::AdminRoutePath::try_from(char::from(0u8).to_string()).is_ok_and(
                |admin_route_path| admin_route_path
                    .as_ref()
                    .chars()
                    .eq(std::iter::once(char::from(0u8)))
            )
        );
    }
}
