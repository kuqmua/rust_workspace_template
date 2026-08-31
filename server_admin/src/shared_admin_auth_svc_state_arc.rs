#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::AsRefOwned,
    newtype::FromInner,
    generate_accessor::Getters,
)]
pub struct SharedAdminAuthSvcStateArc(
    std::sync::Arc<crate::admin_auth_svc_state::AdminAuthSvcState>,
);

impl SharedAdminAuthSvcStateArc {
    #[must_use]
    pub fn from_state(state: crate::admin_auth_svc_state::AdminAuthSvcState) -> Self {
        Self::from(std::sync::Arc::new(state))
    }
}
