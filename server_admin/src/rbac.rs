impl super::AdminAuditAction {
    #[must_use]
    pub fn as_str(self) -> super::StdAdminStrRef<'static> {
        super::StdAdminStrRef::from(match self {
            Self::Create => str_constants::PG_CRUD_CREATE_PERMISSION_ACTION,
            Self::Delete => str_constants::PG_CRUD_DELETE_PERMISSION_ACTION,
            Self::Refresh => str_constants::REFRESH,
            Self::SignIn => str_constants::SIGN_IN,
            Self::SignOut => str_constants::SIGN_OUT,
            Self::Update => str_constants::PG_CRUD_UPDATE_PERMISSION_ACTION,
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

#[cfg(test)]
mod tests {
    #[test]
    fn audit_action_wire_values_cover_every_variant() {
        let actual = [
            super::super::AdminAuditAction::Create.as_str(),
            super::super::AdminAuditAction::Delete.as_str(),
            super::super::AdminAuditAction::Refresh.as_str(),
            super::super::AdminAuditAction::SignIn.as_str(),
            super::super::AdminAuditAction::SignOut.as_str(),
            super::super::AdminAuditAction::Update.as_str(),
        ];
        assert_eq!(
            actual.map(server_admin_core::StdAdminStrRef::get),
            [
                str_constants::PG_CRUD_CREATE_PERMISSION_ACTION,
                str_constants::PG_CRUD_DELETE_PERMISSION_ACTION,
                str_constants::REFRESH,
                str_constants::SIGN_IN,
                str_constants::SIGN_OUT,
                str_constants::PG_CRUD_UPDATE_PERMISSION_ACTION,
            ]
        );
    }

    #[test]
    fn audit_resource_wire_values_cover_every_variant() {
        let actual = [
            super::super::AdminAuditResource::AuditLog.as_str(),
            super::super::AdminAuditResource::Permission.as_str(),
            super::super::AdminAuditResource::Role.as_str(),
            super::super::AdminAuditResource::Session.as_str(),
            super::super::AdminAuditResource::SystemSettings.as_str(),
            super::super::AdminAuditResource::User.as_str(),
        ];
        assert_eq!(
            actual.map(server_admin_core::StdAdminStrRef::get),
            [
                str_constants::AUDIT_LOG_ALT,
                str_constants::PERMISSION,
                str_constants::ROLE,
                str_constants::SESSION,
                str_constants::SYSTEM_SETTINGS,
                str_constants::USER,
            ]
        );
    }
}
