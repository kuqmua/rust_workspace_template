#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub struct AdminSessionBundle {
    pub(crate) access_token: crate::std_admin_access_token::StdAdminAccessToken,
    pub(crate) csrf_token: crate::admin_opaque_token::AdminOpaqueToken,
    pub(crate) refresh_token: crate::admin_refresh_token::AdminRefreshToken,
    pub(crate) session_id: crate::admin_session_id::AdminSessionId,
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
        self.session_id
    }
}
