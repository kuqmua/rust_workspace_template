#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy)]
pub(in super::super) enum AdminAuditResourceId {
    Role(super::super::super::AdminRoleId),
    Session(super::super::super::AdminSessionId),
    SystemSettings,
    User(super::super::super::AdminUserId),
}

impl AdminAuditResourceId {
    pub(in super::super) fn value(self) -> super::super::super::StdAdminString {
        match self {
            Self::User(value) => {
                super::super::super::StdAdminString::from_positive_i64(value.value())
            }
            Self::Role(value) => {
                super::super::super::StdAdminString::from_positive_i64(value.value())
            }
            Self::Session(value) => super::super::super::StdAdminString::from_uuid(value.get()),
            Self::SystemSettings => super::super::super::StdAdminString::system_settings_resource(),
        }
    }
}
