impl super::AdminAuditAction {
    #[must_use]
    pub fn as_str(self) -> super::StdAdminStrRef<'static> {
        super::StdAdminStrRef::from(match self {
            Self::Create => "create",
            Self::Delete => "delete",
            Self::Refresh => "refresh",
            Self::SignIn => "sign_in",
            Self::SignOut => "sign_out",
            Self::Update => "update",
        })
    }
}
impl super::AdminAuditResource {
    #[must_use]
    pub fn as_str(self) -> super::StdAdminStrRef<'static> {
        super::StdAdminStrRef::from(match self {
            Self::AuditLog => "audit_log",
            Self::Permission => "permission",
            Self::Role => "role",
            Self::Session => "session",
            Self::SystemSettings => "system_settings",
            Self::User => "user",
        })
    }
}
impl super::AdminPermission {
    pub const ALL: [Self; 22] = [
        Self::AuditLogRead,
        Self::MetricsRead,
        Self::OpenApiRead,
        Self::PermissionsRead,
        Self::RolePermissionsCreate,
        Self::RolePermissionsDelete,
        Self::RolePermissionsRead,
        Self::RolePermissionsUpdate,
        Self::RolesCreate,
        Self::RolesDelete,
        Self::RolesRead,
        Self::RolesUpdate,
        Self::SystemSettingsRead,
        Self::SystemSettingsUpdate,
        Self::UserRolesCreate,
        Self::UserRolesDelete,
        Self::UserRolesRead,
        Self::UserRolesUpdate,
        Self::UsersCreate,
        Self::UsersDelete,
        Self::UsersRead,
        Self::UsersUpdate,
    ];
    #[must_use]
    pub fn as_str(self) -> super::StdAdminStrRef<'static> {
        super::StdAdminStrRef::from(match self {
            Self::AuditLogRead => server_admin_contract::admin_permission_values::AUDIT_LOG_READ,
            Self::MetricsRead => server_admin_contract::admin_permission_values::METRICS_READ,
            Self::OpenApiRead => server_admin_contract::admin_permission_values::OPEN_API_READ,
            Self::PermissionsRead => {
                server_admin_contract::admin_permission_values::PERMISSIONS_READ
            }
            Self::RolePermissionsCreate => {
                server_admin_contract::admin_permission_values::ROLE_PERMISSIONS_CREATE
            }
            Self::RolePermissionsDelete => {
                server_admin_contract::admin_permission_values::ROLE_PERMISSIONS_DELETE
            }
            Self::RolePermissionsRead => {
                server_admin_contract::admin_permission_values::ROLE_PERMISSIONS_READ
            }
            Self::RolePermissionsUpdate => {
                server_admin_contract::admin_permission_values::ROLE_PERMISSIONS_UPDATE
            }
            Self::RolesCreate => server_admin_contract::admin_permission_values::ROLES_CREATE,
            Self::RolesDelete => server_admin_contract::admin_permission_values::ROLES_DELETE,
            Self::RolesRead => server_admin_contract::admin_permission_values::ROLES_READ,
            Self::RolesUpdate => server_admin_contract::admin_permission_values::ROLES_UPDATE,
            Self::SystemSettingsRead => {
                server_admin_contract::admin_permission_values::SYSTEM_SETTINGS_READ
            }
            Self::SystemSettingsUpdate => {
                server_admin_contract::admin_permission_values::SYSTEM_SETTINGS_UPDATE
            }
            Self::UserRolesCreate => {
                server_admin_contract::admin_permission_values::USER_ROLES_CREATE
            }
            Self::UserRolesDelete => {
                server_admin_contract::admin_permission_values::USER_ROLES_DELETE
            }
            Self::UserRolesRead => server_admin_contract::admin_permission_values::USER_ROLES_READ,
            Self::UserRolesUpdate => {
                server_admin_contract::admin_permission_values::USER_ROLES_UPDATE
            }
            Self::UsersCreate => server_admin_contract::admin_permission_values::USERS_CREATE,
            Self::UsersDelete => server_admin_contract::admin_permission_values::USERS_DELETE,
            Self::UsersRead => server_admin_contract::admin_permission_values::USERS_READ,
            Self::UsersUpdate => server_admin_contract::admin_permission_values::USERS_UPDATE,
        })
    }
}
impl serde::Serialize for super::AdminPermission {
    fn serialize<Serializer>(
        &self,
        serializer: Serializer,
    ) -> Result<Serializer::Ok, Serializer::Error>
    where
        Serializer: serde::Serializer,
    {
        serializer.serialize_str(self.as_str().as_ref())
    }
}
impl TryFrom<&str> for super::AdminPermission {
    type Error = super::AdminPermissionTryFromStrError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::ALL
            .into_iter()
            .find(|permission| permission.as_str().as_ref() == value)
            .ok_or_else(|| super::AdminPermissionTryFromStrError {
                value: super::StdAdminString(value.to_owned()),
            })
    }
}
