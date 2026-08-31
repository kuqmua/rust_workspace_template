#[derive(
    generate_constructor::New,
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    serde::Serialize,
    utoipa::ToSchema,
    generate_accessor::Getters,
)]
pub struct RuntimeAuthenticatedAdmin {
    display_name: server_admin_contract::admin_display_name::AdminDisplayName,
    id: server_admin_core::admin_user_record_id::AdminUserRecordId,
    login: server_admin_contract::admin_login::AdminLogin,
    permissions: crate::admin_auth_permissions::AdminAuthPermissions,
    roles: crate::runtime_admin_role_names::RuntimeAdminRoleNames,
    session_id: crate::admin_session_id::AdminSessionId,
    #[schema(value_type = bool)]
    password_change_required: crate::admin_password_change_required::AdminPasswordChangeRequired,
}
impl RuntimeAuthenticatedAdmin {
    #[must_use]
    pub const fn id(&self) -> server_admin_core::admin_user_record_id::AdminUserRecordId {
        *self.get_id()
    }
    #[must_use]
    pub(crate) const fn password_change_required(
        &self,
    ) -> crate::admin_password_change_required::AdminPasswordChangeRequired {
        *self.get_password_change_required()
    }
}
