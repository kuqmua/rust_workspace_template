#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy)]
pub(crate) enum AdminAuditResourceId {
    Role(server_admin_core::admin_role_id::AdminRoleId),
    Session(crate::admin_session_id::AdminSessionId),
    SystemSettings,
    User(server_admin_core::admin_user_id::AdminUserId),
}

impl AdminAuditResourceId {
    pub(crate) fn value(self) -> server_admin_core::std_admin_string::StdAdminString {
        match self {
            Self::User(value) => {
                server_admin_core::std_admin_string::StdAdminString::from_positive_i64(
                    value.value(),
                )
            }
            Self::Role(value) => {
                server_admin_core::std_admin_string::StdAdminString::from_positive_i64(
                    value.value(),
                )
            }
            Self::Session(value) => {
                server_admin_core::std_admin_string::StdAdminString::from_uuid(value.get())
            }
            Self::SystemSettings => {
                server_admin_core::std_admin_string::StdAdminString::system_settings_resource()
            }
        }
    }
}
