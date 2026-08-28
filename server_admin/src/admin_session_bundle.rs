#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub struct AdminSessionBundle {
    pub(crate) access_token: crate::StdAdminAccessToken,
    pub(crate) csrf_token: crate::AdminOpaqueToken,
    pub(crate) refresh_token: crate::AdminRefreshToken,
    pub(crate) session_id: crate::AdminSessionId,
}
impl AdminSessionBundle {
    #[must_use]
    pub const fn access_token(&self) -> &crate::StdAdminAccessToken {
        &self.access_token
    }
    #[must_use]
    pub const fn csrf_token(&self) -> &crate::AdminOpaqueToken {
        &self.csrf_token
    }
    #[must_use]
    pub const fn refresh_token(&self) -> &crate::AdminRefreshToken {
        &self.refresh_token
    }
    #[must_use]
    pub const fn session_id(&self) -> crate::AdminSessionId {
        self.session_id
    }
}
