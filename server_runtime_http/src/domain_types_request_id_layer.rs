#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Default)]
pub struct RequestIdLayer {
    span_config: Option<super::http_request_span_config::HttpRequestSpanConfig>,
}
impl RequestIdLayer {
    #[must_use]
    pub fn apply(self, router: super::axum_router::AxumRouter) -> super::axum_router::AxumRouter {
        super::axum_router::AxumRouter::from(axum::Router::from(router).layer(
            super::request_id_tower_layer::RequestIdTowerLayer {
                span_config: self.span_config,
            },
        ))
    }

    #[must_use]
    pub const fn with_span_config(
        span_config: super::http_request_span_config::HttpRequestSpanConfig,
    ) -> Self {
        Self {
            span_config: Some(span_config),
        }
    }
}
