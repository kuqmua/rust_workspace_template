#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    proc_macro_newtype_as_ref_owned::AsRefOwned,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_getters::Getters,
)]
pub struct SharedAdminAuthSvcStateArc(
    std::sync::Arc<crate::admin_auth_svc_state::AdminAuthSvcState>,
);

impl SharedAdminAuthSvcStateArc {
    #[must_use]
    pub fn from_state(
        admin_auth_svc_state: crate::admin_auth_svc_state::AdminAuthSvcState,
    ) -> Self {
        Self::from(std::sync::Arc::new(admin_auth_svc_state))
    }
}
