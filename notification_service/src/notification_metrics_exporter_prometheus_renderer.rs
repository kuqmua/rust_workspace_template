#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    proc_macro_newtype_from_inner::FromInner,
)]
pub(crate) struct NotificationMetricsExporterPrometheusRenderer(
    metrics_exporter_prometheus::PrometheusHandle,
);
impl NotificationMetricsExporterPrometheusRenderer {
    pub(crate) fn render(
        &self,
    ) -> Result<
        server_runtime_http::metrics_response_body::MetricsResponseBody,
        server_runtime_http::metrics_response_body_error::MetricsResponseBodyError,
    > {
        server_runtime_http::metrics_response_body::MetricsResponseBody::try_from(self.0.render())
    }
}
