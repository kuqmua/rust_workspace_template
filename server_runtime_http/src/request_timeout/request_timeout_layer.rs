#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub struct RequestTimeoutLayer(pub(super) super::super::RequestTimeoutDuration);

impl RequestTimeoutLayer {
    #[must_use]
    pub fn apply(self, router: super::super::AxumRouter) -> super::super::AxumRouter {
        super::super::AxumRouter::from(
            axum::Router::from(router).layer(super::RequestTimeoutTowerLayer::from(self.0)),
        )
    }
}
