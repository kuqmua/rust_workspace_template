#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    generate_accessor::Getters,
)]
pub struct AdminAccessClaims {
    audience: config_lib::admin_token_audience::AdminTokenAudience,
    expires_at: crate::admin_unix_token_stream::AdminUnixTokenStream,
    issued_at: crate::admin_unix_token_stream::AdminUnixTokenStream,
    issuer: config_lib::admin_token_issuer::AdminTokenIssuer,
    user_id: server_admin_core::admin_user_record_id::AdminUserRecordId,
    session_id: crate::admin_session_id::AdminSessionId,
}

impl AdminAccessClaims {
    #[must_use]
    pub const fn new(
        user_id: server_admin_core::admin_user_record_id::AdminUserRecordId,
        session_id: crate::admin_session_id::AdminSessionId,
        issued_at: crate::admin_unix_token_stream::AdminUnixTokenStream,
        expires_at: crate::admin_unix_token_stream::AdminUnixTokenStream,
        issuer: config_lib::admin_token_issuer::AdminTokenIssuer,
        audience: config_lib::admin_token_audience::AdminTokenAudience,
    ) -> Self {
        Self {
            audience,
            expires_at,
            issued_at,
            issuer,
            user_id,
            session_id,
        }
    }

    #[must_use]
    pub const fn user_id(&self) -> server_admin_core::admin_user_record_id::AdminUserRecordId {
        *self.get_user_id()
    }

    #[must_use]
    pub const fn session_id(&self) -> crate::admin_session_id::AdminSessionId {
        *self.get_session_id()
    }
}
