#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub struct RequestTimeoutLayer(pub(super) crate::request_timeout_duration::RequestTimeoutDuration);

impl RequestTimeoutLayer {
    #[must_use]
    pub fn apply(self, router: crate::axum_router::AxumRouter) -> crate::axum_router::AxumRouter {
        crate::axum_router::AxumRouter::from(
            axum::Router::from(router)
                .layer(crate::request_timeout_tower_layer::RequestTimeoutTowerLayer::from(self.0)),
        )
    }
}
