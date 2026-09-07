#[derive(Debug, thiserror::Error, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) enum AdminFixtureConversionError {
    #[error("{0}")]
    AuditTimestamp(
        server_admin_contract::admin_audit_timestamp::AdminAuditTimestampTryFromStringError,
    ),
    #[error("{0}")]
    DisplayName(server_admin_contract::admin_display_name::AdminDisplayNameTryFromStringError),
    #[error(
        "{}",
        constants_str::WORKSPACE_TEST_RUNNER_ADMIN_FIXTURE_STRING_INVALID
    )]
    Input(crate::admin_fixture_string::AdminFixtureStringTryFromStringError),
    #[error("{0}")]
    Login(server_admin_contract::admin_login::AdminLoginTryFromStringError),
    #[error("{0}")]
    PermissionValue(
        server_admin_contract::admin_permission_value::AdminPermissionValueTryFromStringError,
    ),
    #[error("{0}")]
    RoleName(server_admin_contract::admin_role_name::AdminRoleNameTryFromStringError),
    #[error("{0}")]
    SessionIdentifier(
        server_admin_contract::admin_session_identifier::AdminSessionIdentifierTryFromStringError,
    ),
    #[error("{0}")]
    SessionTimestamp(
        server_admin_contract::admin_session_timestamp::AdminSessionTimestampTryFromStringError,
    ),
    #[error("{0}")]
    Text(server_admin_contract::admin_text::AdminTextTryFromStringError),
}

impl From<server_admin_contract::admin_audit_timestamp::AdminAuditTimestampTryFromStringError>
    for AdminFixtureConversionError
{
    fn from(
        value: server_admin_contract::admin_audit_timestamp::AdminAuditTimestampTryFromStringError,
    ) -> Self {
        Self::AuditTimestamp(value)
    }
}

impl From<server_admin_contract::admin_display_name::AdminDisplayNameTryFromStringError>
    for AdminFixtureConversionError
{
    fn from(
        value: server_admin_contract::admin_display_name::AdminDisplayNameTryFromStringError,
    ) -> Self {
        Self::DisplayName(value)
    }
}

impl From<crate::admin_fixture_string::AdminFixtureStringTryFromStringError>
    for AdminFixtureConversionError
{
    fn from(value: crate::admin_fixture_string::AdminFixtureStringTryFromStringError) -> Self {
        Self::Input(value)
    }
}

impl From<server_admin_contract::admin_login::AdminLoginTryFromStringError>
    for AdminFixtureConversionError
{
    fn from(value: server_admin_contract::admin_login::AdminLoginTryFromStringError) -> Self {
        Self::Login(value)
    }
}

impl From<server_admin_contract::admin_permission_value::AdminPermissionValueTryFromStringError>
    for AdminFixtureConversionError
{
    fn from(
        value: server_admin_contract::admin_permission_value::AdminPermissionValueTryFromStringError,
    ) -> Self {
        Self::PermissionValue(value)
    }
}

impl From<server_admin_contract::admin_role_name::AdminRoleNameTryFromStringError>
    for AdminFixtureConversionError
{
    fn from(
        value: server_admin_contract::admin_role_name::AdminRoleNameTryFromStringError,
    ) -> Self {
        Self::RoleName(value)
    }
}

impl From<server_admin_contract::admin_session_identifier::AdminSessionIdentifierTryFromStringError>
    for AdminFixtureConversionError
{
    fn from(
        value: server_admin_contract::admin_session_identifier::AdminSessionIdentifierTryFromStringError,
    ) -> Self {
        Self::SessionIdentifier(value)
    }
}

impl From<server_admin_contract::admin_session_timestamp::AdminSessionTimestampTryFromStringError>
    for AdminFixtureConversionError
{
    fn from(
        value: server_admin_contract::admin_session_timestamp::AdminSessionTimestampTryFromStringError,
    ) -> Self {
        Self::SessionTimestamp(value)
    }
}

impl From<server_admin_contract::admin_text::AdminTextTryFromStringError>
    for AdminFixtureConversionError
{
    fn from(value: server_admin_contract::admin_text::AdminTextTryFromStringError) -> Self {
        Self::Text(value)
    }
}
