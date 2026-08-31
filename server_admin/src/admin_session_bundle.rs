#[derive(
    generate_constructor::New,
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    generate_accessor::Getters,
)]
pub struct AdminSessionBundle {
    access_token: crate::std_admin_access_token::StdAdminAccessToken,
    csrf_token: crate::admin_opaque_token::AdminOpaqueToken,
    refresh_token: crate::admin_refresh_token::AdminRefreshToken,
    session_id: crate::admin_session_id::AdminSessionId,
}
impl AdminSessionBundle {
    #[must_use]
    pub const fn access_token(&self) -> &crate::std_admin_access_token::StdAdminAccessToken {
        &self.access_token
    }
    #[must_use]
    pub const fn csrf_token(&self) -> &crate::admin_opaque_token::AdminOpaqueToken {
        &self.csrf_token
    }
    #[must_use]
    pub const fn refresh_token(&self) -> &crate::admin_refresh_token::AdminRefreshToken {
        &self.refresh_token
    }
    #[must_use]
    pub const fn session_id(&self) -> crate::admin_session_id::AdminSessionId {
        *self.get_session_id()
    }
}
