#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Serialize, utoipa::ToSchema)]
pub struct AuthenticatedAdmin {
    pub(super) display_name: super::super::AdminDisplayName,
    pub(super) id: super::super::AdminUserId,
    pub(super) login: super::super::AdminLogin,
    pub(super) permissions: super::super::AdminAuthPermissions,
    pub(super) roles: super::super::AdminRoleNames,
    pub(super) session_id: super::super::AdminSessionId,
    #[schema(value_type = bool)]
    pub(super) password_change_required: super::super::AdminPasswordChangeRequired,
}
impl AuthenticatedAdmin {
    #[must_use]
    pub const fn id(&self) -> super::super::AdminUserId {
        self.id
    }
    #[must_use]
    pub(crate) const fn password_change_required(
        &self,
    ) -> super::super::AdminPasswordChangeRequired {
        self.password_change_required
    }
}
