#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Default)]
pub struct RequestIdLayer {
    span_config: Option<super::http_request_span_config::HttpRequestSpanConfig>,
}
impl RequestIdLayer {
    #[must_use]
    pub fn apply(
        self,
        axum_router: super::axum_router::AxumRouter,
    ) -> super::axum_router::AxumRouter {
        super::axum_router::AxumRouter::from(axum::Router::from(axum_router).layer(
            super::request_id_tower_layer::RequestIdTowerLayer::new(self.span_config),
        ))
    }

    #[must_use]
    pub const fn with_span_config(
        http_request_span_config: super::http_request_span_config::HttpRequestSpanConfig,
    ) -> Self {
        Self {
            span_config: Some(http_request_span_config),
        }
    }
}
