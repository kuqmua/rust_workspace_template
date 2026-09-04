#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    proc_macro_getters::Getters,
    proc_macro_new::New,
)]
pub struct AdminAccessClaims {
    #[constructor(order = 5)]
    audience: config_lib::admin_token_audience::AdminTokenAudience,
    #[constructor(order = 3)]
    expires_at: crate::admin_unix_token_stream::AdminUnixTokenStream,
    #[constructor(order = 2)]
    issued_at: crate::admin_unix_token_stream::AdminUnixTokenStream,
    #[constructor(order = 4)]
    issuer: config_lib::admin_token_issuer::AdminTokenIssuer,
    #[constructor(order = 0)]
    user_id: server_admin_core::admin_user_record_id::AdminUserRecordId,
    #[constructor(order = 1)]
    session_id: crate::admin_session_id::AdminSessionId,
}

impl AdminAccessClaims {
    #[must_use]
    pub const fn user_id(&self) -> server_admin_core::admin_user_record_id::AdminUserRecordId {
        *self.get_user_id()
    }

    #[must_use]
    pub const fn session_id(&self) -> crate::admin_session_id::AdminSessionId {
        *self.get_session_id()
    }
}
