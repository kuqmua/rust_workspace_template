impl crate::admin_audit_action::AdminAuditAction {
    #[must_use]
    pub fn as_str(self) -> server_admin_core::std_admin_str_ref::StdAdminStrRef<'static> {
        server_admin_core::std_admin_str_ref::StdAdminStrRef::from(match self {
            Self::Create => constants_str::PG_CRUD_CREATE_PERMISSION_ACTION,
            Self::Delete => constants_str::PG_CRUD_DELETE_PERMISSION_ACTION,
            Self::Refresh => constants_str::REFRESH,
            Self::SignIn => constants_str::SIGN_IN,
            Self::SignOut => constants_str::SIGN_OUT,
            Self::Update => constants_str::PG_CRUD_UPDATE_PERMISSION_ACTION,
        })
    }
}
impl crate::admin_audit_resource::AdminAuditResource {
    #[must_use]
    pub fn as_str(self) -> server_admin_core::std_admin_str_ref::StdAdminStrRef<'static> {
        server_admin_core::std_admin_str_ref::StdAdminStrRef::from(match self {
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
            crate::admin_audit_action::AdminAuditAction::Create.as_str(),
            crate::admin_audit_action::AdminAuditAction::Delete.as_str(),
            crate::admin_audit_action::AdminAuditAction::Refresh.as_str(),
            crate::admin_audit_action::AdminAuditAction::SignIn.as_str(),
            crate::admin_audit_action::AdminAuditAction::SignOut.as_str(),
            crate::admin_audit_action::AdminAuditAction::Update.as_str(),
        ];
        assert_eq!(
            actual.map(server_admin_core::std_admin_str_ref::StdAdminStrRef::get),
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
            crate::admin_audit_resource::AdminAuditResource::AuditLog.as_str(),
            crate::admin_audit_resource::AdminAuditResource::Permission.as_str(),
            crate::admin_audit_resource::AdminAuditResource::Role.as_str(),
            crate::admin_audit_resource::AdminAuditResource::Session.as_str(),
            crate::admin_audit_resource::AdminAuditResource::SystemSettings.as_str(),
            crate::admin_audit_resource::AdminAuditResource::User.as_str(),
        ];
        assert_eq!(
            actual.map(server_admin_core::std_admin_str_ref::StdAdminStrRef::get),
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
