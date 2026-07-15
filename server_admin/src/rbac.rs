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
            Self::AuditLogRead => contract_constants::admin_permission_values::AUDIT_LOG_READ,
            Self::MetricsRead => contract_constants::admin_permission_values::METRICS_READ,
            Self::OpenApiRead => contract_constants::admin_permission_values::OPEN_API_READ,
            Self::PermissionsRead => contract_constants::admin_permission_values::PERMISSIONS_READ,
            Self::RolePermissionsCreate => {
                contract_constants::admin_permission_values::ROLE_PERMISSIONS_CREATE
            }
            Self::RolePermissionsDelete => {
                contract_constants::admin_permission_values::ROLE_PERMISSIONS_DELETE
            }
            Self::RolePermissionsRead => {
                contract_constants::admin_permission_values::ROLE_PERMISSIONS_READ
            }
            Self::RolePermissionsUpdate => {
                contract_constants::admin_permission_values::ROLE_PERMISSIONS_UPDATE
            }
            Self::RolesCreate => contract_constants::admin_permission_values::ROLES_CREATE,
            Self::RolesDelete => contract_constants::admin_permission_values::ROLES_DELETE,
            Self::RolesRead => contract_constants::admin_permission_values::ROLES_READ,
            Self::RolesUpdate => contract_constants::admin_permission_values::ROLES_UPDATE,
            Self::SystemSettingsRead => {
                contract_constants::admin_permission_values::SYSTEM_SETTINGS_READ
            }
            Self::SystemSettingsUpdate => {
                contract_constants::admin_permission_values::SYSTEM_SETTINGS_UPDATE
            }
            Self::UserRolesCreate => contract_constants::admin_permission_values::USER_ROLES_CREATE,
            Self::UserRolesDelete => contract_constants::admin_permission_values::USER_ROLES_DELETE,
            Self::UserRolesRead => contract_constants::admin_permission_values::USER_ROLES_READ,
            Self::UserRolesUpdate => contract_constants::admin_permission_values::USER_ROLES_UPDATE,
            Self::UsersCreate => contract_constants::admin_permission_values::USERS_CREATE,
            Self::UsersDelete => contract_constants::admin_permission_values::USERS_DELETE,
            Self::UsersRead => contract_constants::admin_permission_values::USERS_READ,
            Self::UsersUpdate => contract_constants::admin_permission_values::USERS_UPDATE,
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
