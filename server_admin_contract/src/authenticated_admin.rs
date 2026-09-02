#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
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
    #[getters(skip)]
    permissions: crate::admin_permission_values::AdminPermissionValues,
    #[getters(skip)]
    roles: crate::admin_role_names::AdminRoleNames,
}
impl AuthenticatedAdmin {
    #[must_use]
    pub const fn new(
        admin_display_name: crate::admin_display_name::AdminDisplayName,
        admin_user_id: crate::admin_user_id::AdminUserId,
        admin_login: crate::admin_login::AdminLogin,
        admin_permission_values: crate::admin_permission_values::AdminPermissionValues,
        admin_role_names: crate::admin_role_names::AdminRoleNames,
    ) -> Self {
        Self {
            display_name: admin_display_name,
            id: admin_user_id,
            login: admin_login,
            permissions: admin_permission_values,
            roles: admin_role_names,
        }
    }

    #[must_use]
    pub fn permissions(&self) -> &[crate::admin_permission_value::AdminPermissionValue] {
        self.permissions.as_ref()
    }

    #[must_use]
    pub const fn roles(&self) -> &[crate::admin_role_name::AdminRoleName] {
        self.roles.as_slice()
    }
    #[must_use]
    pub fn has_permission(
        &self,
        admin_permission: crate::admin_permission::AdminPermission,
    ) -> crate::admin_bool::AdminBool {
        let required = admin_permission.as_str();
        crate::admin_bool::AdminBool::from(
            self.permissions
                .as_ref()
                .iter()
                .any(|value| value.as_ref() == required.get()),
        )
    }
    #[must_use]
    pub fn can_access(
        &self,
        admin_page: crate::admin_page::AdminPage,
    ) -> crate::admin_bool::AdminBool {
        crate::admin_bool::AdminBool::from(match admin_page.authentication() {
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
