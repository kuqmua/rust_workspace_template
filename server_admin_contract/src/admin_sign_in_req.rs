#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    generate_constructor::New,
)]
#[serde(deny_unknown_fields)]
pub struct AdminSignInReq {
    login: crate::admin_login::AdminLogin,
    password: crate::admin_password::AdminPassword,
}
impl AdminSignInReq {
    #[must_use]
    pub const fn login(&self) -> &crate::admin_login::AdminLogin {
        &self.login
    }
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        crate::admin_login::AdminLogin,
        crate::admin_password::AdminPassword,
    ) {
        (self.login, self.password)
    }
    #[must_use]
    pub const fn password(&self) -> &crate::admin_password::AdminPassword {
        &self.password
    }
}
