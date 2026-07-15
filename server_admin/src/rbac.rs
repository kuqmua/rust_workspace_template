impl super::AdminAuditAction {
    #[must_use]
    pub fn as_str(self) -> super::StdAdminStrRef<'static> {
        super::StdAdminStrRef::from(match self {
            Self::Create => str_constants::pg_crud::CREATE_PERMISSION_ACTION,
            Self::Delete => str_constants::pg_crud::DELETE_PERMISSION_ACTION,
            Self::Refresh => str_constants::REFRESH,
            Self::SignIn => str_constants::SIGN_IN,
            Self::SignOut => str_constants::SIGN_OUT,
            Self::Update => str_constants::pg_crud::UPDATE_PERMISSION_ACTION,
        })
    }
}
impl super::AdminAuditResource {
    #[must_use]
    pub fn as_str(self) -> super::StdAdminStrRef<'static> {
        super::StdAdminStrRef::from(match self {
            Self::AuditLog => str_constants::AUDIT_LOG_ALT,
            Self::Permission => str_constants::PERMISSION,
            Self::Role => str_constants::ROLE,
            Self::Session => str_constants::SESSION,
            Self::SystemSettings => str_constants::SYSTEM_SETTINGS,
            Self::User => str_constants::USER,
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
            Self::AuditLogRead => str_constants::admin_permission_values::AUDIT_LOG_READ,
            Self::MetricsRead => str_constants::admin_permission_values::METRICS_READ,
            Self::OpenApiRead => str_constants::admin_permission_values::OPEN_API_READ,
            Self::PermissionsRead => str_constants::admin_permission_values::PERMISSIONS_READ,
            Self::RolePermissionsCreate => {
                str_constants::admin_permission_values::ROLE_PERMISSIONS_CREATE
            }
            Self::RolePermissionsDelete => {
                str_constants::admin_permission_values::ROLE_PERMISSIONS_DELETE
            }
            Self::RolePermissionsRead => {
                str_constants::admin_permission_values::ROLE_PERMISSIONS_READ
            }
            Self::RolePermissionsUpdate => {
                str_constants::admin_permission_values::ROLE_PERMISSIONS_UPDATE
            }
            Self::RolesCreate => str_constants::admin_permission_values::ROLES_CREATE,
            Self::RolesDelete => str_constants::admin_permission_values::ROLES_DELETE,
            Self::RolesRead => str_constants::admin_permission_values::ROLES_READ,
            Self::RolesUpdate => str_constants::admin_permission_values::ROLES_UPDATE,
            Self::SystemSettingsRead => {
                str_constants::admin_permission_values::SYSTEM_SETTINGS_READ
            }
            Self::SystemSettingsUpdate => {
                str_constants::admin_permission_values::SYSTEM_SETTINGS_UPDATE
            }
            Self::UserRolesCreate => str_constants::admin_permission_values::USER_ROLES_CREATE,
            Self::UserRolesDelete => str_constants::admin_permission_values::USER_ROLES_DELETE,
            Self::UserRolesRead => str_constants::admin_permission_values::USER_ROLES_READ,
            Self::UserRolesUpdate => str_constants::admin_permission_values::USER_ROLES_UPDATE,
            Self::UsersCreate => str_constants::admin_permission_values::USERS_CREATE,
            Self::UsersDelete => str_constants::admin_permission_values::USERS_DELETE,
            Self::UsersRead => str_constants::admin_permission_values::USERS_READ,
            Self::UsersUpdate => str_constants::admin_permission_values::USERS_UPDATE,
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
