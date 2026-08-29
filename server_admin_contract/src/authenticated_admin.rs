#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
pub struct AuthenticatedAdmin {
    display_name: crate::admin_display_name::AdminDisplayName,
    id: crate::admin_user_id::AdminUserId,
    login: crate::admin_login::AdminLogin,
    permissions: crate::admin_permission_values::AdminPermissionValues,
    roles: crate::admin_role_names::AdminRoleNames,
}
impl AuthenticatedAdmin {
    #[must_use]
    pub const fn new(
        display_name: crate::admin_display_name::AdminDisplayName,
        id: crate::admin_user_id::AdminUserId,
        login: crate::admin_login::AdminLogin,
        permissions: crate::admin_permission_values::AdminPermissionValues,
        roles: crate::admin_role_names::AdminRoleNames,
    ) -> Self {
        Self {
            display_name,
            id,
            login,
            permissions,
            roles,
        }
    }
    #[must_use]
    pub const fn display_name(&self) -> &crate::admin_display_name::AdminDisplayName {
        &self.display_name
    }
    #[must_use]
    pub fn permissions(&self) -> &[crate::admin_permission_value::AdminPermissionValue] {
        self.permissions.as_ref()
    }
    #[must_use]
    pub const fn login(&self) -> &crate::admin_login::AdminLogin {
        &self.login
    }
    #[must_use]
    pub const fn roles(&self) -> &[crate::admin_role_name::AdminRoleName] {
        self.roles.as_slice()
    }
    #[must_use]
    pub fn has_permission(
        &self,
        permission: crate::admin_permission::AdminPermission,
    ) -> crate::admin_bool::AdminBool {
        let required = permission.as_str();
        crate::admin_bool::AdminBool::from(
            self.permissions
                .as_ref()
                .iter()
                .any(|value| value.as_ref() == required.get()),
        )
    }
    #[must_use]
    pub fn can_access(&self, page: crate::admin_page::AdminPage) -> crate::admin_bool::AdminBool {
        crate::admin_bool::AdminBool::from(match page.authentication() {
            frontend_contract::authentication_requirement::AuthenticationRequirement::Authenticated
            | frontend_contract::authentication_requirement::AuthenticationRequirement::Public => true,
            frontend_contract::authentication_requirement::AuthenticationRequirement::Permission(required) => self
                .permissions
                .as_ref()
                .iter()
                .any(|value| value.as_ref() == required.as_ref()),
        })
    }
}
