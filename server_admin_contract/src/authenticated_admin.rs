#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    proc_macro_new::New,
)]
pub struct AuthenticatedAdmin {
    display_name: crate::admin_display_name::AdminDisplayName,
    id: crate::admin_user_id::AdminUserId,
    login: crate::admin_login::AdminLogin,
    #[getters(skip)]
    rules: crate::admin_rule_values::AdminRuleValues,
    #[getters(skip)]
    roles: crate::admin_role_names::AdminRoleNames,
}
impl AuthenticatedAdmin {
    #[must_use]
    pub fn rules(&self) -> &[crate::admin_rule_value::AdminRuleValue] {
        self.rules.as_ref()
    }

    #[must_use]
    pub const fn roles(&self) -> &[crate::admin_role_name::AdminRoleName] {
        self.roles.as_slice()
    }
    #[must_use]
    pub fn has_rule(
        &self,
        admin_rule: crate::admin_rule::AdminRule,
    ) -> crate::admin_bool::AdminBool {
        let required = admin_rule.as_str();
        crate::admin_bool::AdminBool::from(
            self.rules
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
            frontend_contract::authentication_requirement::AuthenticationRequirement::Rule(required) => self
                .rules
                .as_ref()
                .iter()
                .any(|value| value.as_ref() == required.as_ref()),
        })
    }
}
