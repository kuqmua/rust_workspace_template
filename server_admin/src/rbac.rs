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
