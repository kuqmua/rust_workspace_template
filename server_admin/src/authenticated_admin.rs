#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Serialize, utoipa::ToSchema)]
pub struct AuthenticatedAdmin {
    pub(crate) display_name: server_admin_contract::admin_display_name::AdminDisplayName,
    pub(crate) id: server_admin_core::admin_user_id::AdminUserId,
    pub(crate) login: server_admin_contract::admin_login::AdminLogin,
    pub(crate) permissions: crate::admin_auth_permissions::AdminAuthPermissions,
    pub(crate) roles: crate::admin_role_names::AdminRoleNames,
    pub(crate) session_id: crate::admin_session_id::AdminSessionId,
    #[schema(value_type = bool)]
    pub(crate) password_change_required:
        crate::admin_password_change_required::AdminPasswordChangeRequired,
}
impl AuthenticatedAdmin {
    #[must_use]
    pub const fn id(&self) -> server_admin_core::admin_user_id::AdminUserId {
        self.id
    }
    #[must_use]
    pub(crate) const fn password_change_required(
        &self,
    ) -> crate::admin_password_change_required::AdminPasswordChangeRequired {
        self.password_change_required
    }
}
