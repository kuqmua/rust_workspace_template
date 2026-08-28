#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Serialize, utoipa::ToSchema)]
pub struct AuthenticatedAdmin {
    pub(crate) display_name: crate::AdminDisplayName,
    pub(crate) id: crate::AdminUserId,
    pub(crate) login: crate::AdminLogin,
    pub(crate) permissions: crate::AdminAuthPermissions,
    pub(crate) roles: crate::AdminRoleNames,
    pub(crate) session_id: crate::AdminSessionId,
    #[schema(value_type = bool)]
    pub(crate) password_change_required: crate::AdminPasswordChangeRequired,
}
impl AuthenticatedAdmin {
    #[must_use]
    pub const fn id(&self) -> crate::AdminUserId {
        self.id
    }
    #[must_use]
    pub(crate) const fn password_change_required(&self) -> crate::AdminPasswordChangeRequired {
        self.password_change_required
    }
}
