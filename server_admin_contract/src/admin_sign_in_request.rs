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
#[serde(deny_unknown_fields)]
pub struct AdminSignInRequest {
    login: crate::admin_login::AdminLogin,
    password: crate::admin_password::AdminPassword,
}
impl AdminSignInRequest {
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        crate::admin_login::AdminLogin,
        crate::admin_password::AdminPassword,
    ) {
        (self.login, self.password)
    }
}
