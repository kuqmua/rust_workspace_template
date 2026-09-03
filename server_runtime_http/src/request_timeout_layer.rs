#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_newtype_from_inner::FromInner,
)]
pub struct RequestTimeoutLayer(crate::request_timeout_duration::RequestTimeoutDuration);

impl RequestTimeoutLayer {
    pub(crate) const fn duration(self) -> crate::request_timeout_duration::RequestTimeoutDuration {
        self.0
    }

    #[must_use]
    pub fn apply(
        self,
        axum_router: crate::axum_router::AxumRouter,
    ) -> crate::axum_router::AxumRouter {
        crate::axum_router::AxumRouter::from(axum::Router::from(axum_router).layer(
            crate::request_timeout_tower_layer::RequestTimeoutTowerLayer::from(self.duration()),
        ))
    }
}
