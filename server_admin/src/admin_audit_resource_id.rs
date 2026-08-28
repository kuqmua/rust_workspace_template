#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy)]
pub(crate) enum AdminAuditResourceId {
    Role(crate::AdminRoleId),
    Session(crate::AdminSessionId),
    SystemSettings,
    User(crate::AdminUserId),
}

impl AdminAuditResourceId {
    pub(crate) fn value(self) -> crate::StdAdminString {
        match self {
            Self::User(value) => crate::StdAdminString::from_positive_i64(value.value()),
            Self::Role(value) => crate::StdAdminString::from_positive_i64(value.value()),
            Self::Session(value) => crate::StdAdminString::from_uuid(value.get()),
            Self::SystemSettings => crate::StdAdminString::system_settings_resource(),
        }
    }
}
