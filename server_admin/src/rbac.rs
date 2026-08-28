impl crate::AdminAuditAction {
    #[must_use]
    pub fn as_str(self) -> crate::StdAdminStrRef<'static> {
        crate::StdAdminStrRef::from(match self {
            Self::Create => constants_str::PG_CRUD_CREATE_PERMISSION_ACTION,
            Self::Delete => constants_str::PG_CRUD_DELETE_PERMISSION_ACTION,
            Self::Refresh => constants_str::REFRESH,
            Self::SignIn => constants_str::SIGN_IN,
            Self::SignOut => constants_str::SIGN_OUT,
            Self::Update => constants_str::PG_CRUD_UPDATE_PERMISSION_ACTION,
        })
    }
}
impl crate::AdminAuditResource {
    #[must_use]
    pub fn as_str(self) -> crate::StdAdminStrRef<'static> {
        crate::StdAdminStrRef::from(match self {
            Self::AuditLog => constants_str::AUDIT_LOG_ALT,
            Self::Permission => constants_str::PERMISSION,
            Self::Role => constants_str::ROLE,
            Self::Session => constants_str::SESSION,
            Self::SystemSettings => constants_str::SYSTEM_SETTINGS,
            Self::User => constants_str::USER,
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn audit_action_wire_values_cover_every_variant() {
        let actual = [
            crate::AdminAuditAction::Create.as_str(),
            crate::AdminAuditAction::Delete.as_str(),
            crate::AdminAuditAction::Refresh.as_str(),
            crate::AdminAuditAction::SignIn.as_str(),
            crate::AdminAuditAction::SignOut.as_str(),
            crate::AdminAuditAction::Update.as_str(),
        ];
        assert_eq!(
            actual.map(server_admin_core::domain_types::StdAdminStrRef::get),
            [
                constants_str::PG_CRUD_CREATE_PERMISSION_ACTION,
                constants_str::PG_CRUD_DELETE_PERMISSION_ACTION,
                constants_str::REFRESH,
                constants_str::SIGN_IN,
                constants_str::SIGN_OUT,
                constants_str::PG_CRUD_UPDATE_PERMISSION_ACTION,
            ]
        );
    }

    #[test]
    fn audit_resource_wire_values_cover_every_variant() {
        let actual = [
            crate::AdminAuditResource::AuditLog.as_str(),
            crate::AdminAuditResource::Permission.as_str(),
            crate::AdminAuditResource::Role.as_str(),
            crate::AdminAuditResource::Session.as_str(),
            crate::AdminAuditResource::SystemSettings.as_str(),
            crate::AdminAuditResource::User.as_str(),
        ];
        assert_eq!(
            actual.map(server_admin_core::domain_types::StdAdminStrRef::get),
            [
                constants_str::AUDIT_LOG_ALT,
                constants_str::PERMISSION,
                constants_str::ROLE,
                constants_str::SESSION,
                constants_str::SYSTEM_SETTINGS,
                constants_str::USER,
            ]
        );
    }
}
