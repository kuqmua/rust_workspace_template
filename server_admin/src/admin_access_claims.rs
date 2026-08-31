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
    aud: config_lib::admin_token_audience::AdminTokenAudience,
    exp: crate::admin_unix_token_stream::AdminUnixTokenStream,
    iat: crate::admin_unix_token_stream::AdminUnixTokenStream,
    iss: config_lib::admin_token_issuer::AdminTokenIssuer,
    sub: server_admin_core::admin_user_record_id::AdminUserRecordId,
    jti: crate::admin_session_id::AdminSessionId,
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
            aud: audience,
            exp: expires_at,
            iat: issued_at,
            iss: issuer,
            jti: session_id,
            sub: user_id,
        }
    }

    #[must_use]
    pub const fn user_id(&self) -> server_admin_core::admin_user_record_id::AdminUserRecordId {
        *self.get_sub()
    }

    #[must_use]
    pub const fn session_id(&self) -> crate::admin_session_id::AdminSessionId {
        *self.get_jti()
    }
}
