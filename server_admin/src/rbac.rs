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
    #[must_use]
    pub fn as_str(self) -> super::StdAdminStrRef<'static> {
        super::StdAdminStrRef::from(match self {
            Self::AuditLogRead => "audit_log:read",
            Self::MetricsRead => "metrics:read",
            Self::OpenApiRead => "openapi:read",
            Self::PermissionsRead => "permissions:read",
            Self::RolePermissionsCreate => "role_permissions:create",
            Self::RolePermissionsDelete => "role_permissions:delete",
            Self::RolePermissionsRead => "role_permissions:read",
            Self::RolePermissionsUpdate => "role_permissions:update",
            Self::RolesCreate => "roles:create",
            Self::RolesDelete => "roles:delete",
            Self::RolesRead => "roles:read",
            Self::RolesUpdate => "roles:update",
            Self::SystemSettingsRead => "system_settings:read",
            Self::SystemSettingsUpdate => "system_settings:update",
            Self::UserRolesCreate => "user_roles:create",
            Self::UserRolesDelete => "user_roles:delete",
            Self::UserRolesRead => "user_roles:read",
            Self::UserRolesUpdate => "user_roles:update",
            Self::UsersCreate => "users:create",
            Self::UsersDelete => "users:delete",
            Self::UsersRead => "users:read",
            Self::UsersUpdate => "users:update",
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
        match value {
            "audit_log:read" => Ok(Self::AuditLogRead),
            "metrics:read" => Ok(Self::MetricsRead),
            "openapi:read" => Ok(Self::OpenApiRead),
            "permissions:read" => Ok(Self::PermissionsRead),
            "role_permissions:create" => Ok(Self::RolePermissionsCreate),
            "role_permissions:delete" => Ok(Self::RolePermissionsDelete),
            "role_permissions:read" => Ok(Self::RolePermissionsRead),
            "role_permissions:update" => Ok(Self::RolePermissionsUpdate),
            "roles:create" => Ok(Self::RolesCreate),
            "roles:delete" => Ok(Self::RolesDelete),
            "roles:read" => Ok(Self::RolesRead),
            "roles:update" => Ok(Self::RolesUpdate),
            "system_settings:read" => Ok(Self::SystemSettingsRead),
            "system_settings:update" => Ok(Self::SystemSettingsUpdate),
            "user_roles:create" => Ok(Self::UserRolesCreate),
            "user_roles:delete" => Ok(Self::UserRolesDelete),
            "user_roles:read" => Ok(Self::UserRolesRead),
            "user_roles:update" => Ok(Self::UserRolesUpdate),
            "users:create" => Ok(Self::UsersCreate),
            "users:delete" => Ok(Self::UsersDelete),
            "users:read" => Ok(Self::UsersRead),
            "users:update" => Ok(Self::UsersUpdate),
            _ => Err(super::AdminPermissionTryFromStrError {
                value: super::StdAdminString(value.to_owned()),
            }),
        }
    }
}
