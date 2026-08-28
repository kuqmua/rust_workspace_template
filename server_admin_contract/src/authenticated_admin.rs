#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
pub struct AuthenticatedAdmin {
    display_name: crate::domain_types::AdminDisplayName,
    id: crate::domain_types::AdminUserId,
    login: crate::domain_types::AdminLogin,
    permissions: crate::domain_types::AdminPermissionValues,
    roles: crate::domain_types::AdminRoleNames,
}
impl AuthenticatedAdmin {
    #[must_use]
    pub const fn new(
        display_name: crate::domain_types::AdminDisplayName,
        id: crate::domain_types::AdminUserId,
        login: crate::domain_types::AdminLogin,
        permissions: crate::domain_types::AdminPermissionValues,
        roles: crate::domain_types::AdminRoleNames,
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
    pub const fn display_name(&self) -> &crate::domain_types::AdminDisplayName {
        &self.display_name
    }
    #[must_use]
    pub fn permissions(&self) -> &[crate::domain_types::AdminPermissionValue] {
        self.permissions.as_ref()
    }
    #[must_use]
    pub const fn login(&self) -> &crate::domain_types::AdminLogin {
        &self.login
    }
    #[must_use]
    pub const fn roles(&self) -> &[crate::domain_types::AdminRoleName] {
        self.roles.as_slice()
    }
    #[must_use]
    pub fn has_permission(
        &self,
        permission: crate::domain_types::AdminPermission,
    ) -> crate::domain_types::AdminBool {
        let required = permission.as_str();
        crate::domain_types::AdminBool::from(
            self.permissions
                .as_ref()
                .iter()
                .any(|value| value.as_ref() == required.get()),
        )
    }
    #[must_use]
    pub fn can_access(
        &self,
        page: crate::domain_types::AdminPage,
    ) -> crate::domain_types::AdminBool {
        crate::domain_types::AdminBool::from(match page.authentication() {
            frontend_contract::domain_types::AuthenticationRequirement::Authenticated
            | frontend_contract::domain_types::AuthenticationRequirement::Public => true,
            frontend_contract::domain_types::AuthenticationRequirement::Permission(required) => {
                self.permissions
                    .as_ref()
                    .iter()
                    .any(|value| value.as_ref() == required.as_ref())
            }
        })
    }
}
