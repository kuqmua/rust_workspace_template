#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::AsRefOwned,
    newtype::FromInner,
)]
pub struct SharedAdminAuthSvcStateArc(
    pub(crate) std::sync::Arc<crate::admin_auth_svc_state::AdminAuthSvcState>,
);

impl SharedAdminAuthSvcStateArc {
    #[must_use]
    pub fn from_state(state: crate::admin_auth_svc_state::AdminAuthSvcState) -> Self {
        Self::from(std::sync::Arc::new(state))
    }
}
