#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub(crate) struct MetricsExporterPrometheusRenderer(metrics_exporter_prometheus::PrometheusHandle);
