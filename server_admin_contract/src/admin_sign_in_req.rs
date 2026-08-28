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
    login: crate::domain_types::AdminLogin,
    password: crate::domain_types::AdminPassword,
}
impl AdminSignInReq {
    #[must_use]
    pub const fn login(&self) -> &crate::domain_types::AdminLogin {
        &self.login
    }
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        crate::domain_types::AdminLogin,
        crate::domain_types::AdminPassword,
    ) {
        (self.login, self.password)
    }
    #[must_use]
    pub const fn password(&self) -> &crate::domain_types::AdminPassword {
        &self.password
    }
}
