#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_into_inner_from::IntoInnerFrom,
)]
pub(crate) struct AxumMetricsExporterPrometheusRenderer(
    crate::metrics_exporter_prometheus_renderer::MetricsExporterPrometheusRenderer,
);

impl
    axum::extract::FromRequestParts<
        crate::metrics_exporter_prometheus_renderer::MetricsExporterPrometheusRenderer,
    > for AxumMetricsExporterPrometheusRenderer
{
    type Rejection = std::convert::Infallible;

    fn from_request_parts(
        _parts: &mut axum::http::request::Parts,
        metrics_exporter_prometheus_renderer: &crate::metrics_exporter_prometheus_renderer::MetricsExporterPrometheusRenderer,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> {
        std::future::ready(Ok(Self::from(metrics_exporter_prometheus_renderer.clone())))
    }
}
