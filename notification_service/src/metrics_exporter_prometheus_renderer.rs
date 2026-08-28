#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, newtype::FromInner)]
pub(crate) struct MetricsExporterPrometheusRenderer(metrics_exporter_prometheus::PrometheusHandle);
impl MetricsExporterPrometheusRenderer {
    pub(crate) fn render(
        &self,
    ) -> Result<
        server_runtime_http::domain_types::MetricsResponseBody,
        server_runtime_http::domain_types::MetricsResponseBodyError,
    > {
        server_runtime_http::domain_types::MetricsResponseBody::try_from(self.0.render())
    }
}
