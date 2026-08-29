#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, newtype::FromInner)]
pub(crate) struct MetricsExporterPrometheusRenderer(metrics_exporter_prometheus::PrometheusHandle);
impl MetricsExporterPrometheusRenderer {
    pub(crate) fn render(
        &self,
    ) -> Result<
        server_runtime_http::metrics_response_body::MetricsResponseBody,
        server_runtime_http::metrics_response_body_error::MetricsResponseBodyError,
    > {
        server_runtime_http::metrics_response_body::MetricsResponseBody::try_from(self.0.render())
    }
}
