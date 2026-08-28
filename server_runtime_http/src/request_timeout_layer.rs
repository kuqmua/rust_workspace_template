#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub struct RequestTimeoutLayer(pub(super) crate::RequestTimeoutDuration);

impl RequestTimeoutLayer {
    #[must_use]
    pub fn apply(self, router: crate::AxumRouter) -> crate::AxumRouter {
        crate::AxumRouter::from(
            axum::Router::from(router).layer(super::RequestTimeoutTowerLayer::from(self.0)),
        )
    }
}
