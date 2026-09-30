#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    proc_macro_newtype_as_ref_owned::AsRefOwned,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_getters::Getters,
)]
pub struct SharedAdminAuthServiceStateArc(
    std::sync::Arc<crate::admin_auth_service_state::AdminAuthServiceState>,
);

impl SharedAdminAuthServiceStateArc {
    #[must_use]
    pub fn from_state(
        admin_auth_service_state: crate::admin_auth_service_state::AdminAuthServiceState,
    ) -> Self {
        Self::from(std::sync::Arc::new(admin_auth_service_state))
    }
}
