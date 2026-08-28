#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner, newtype::Display,
)]
pub(crate) struct MetricsExporterPrometheusNotificationBuildError(
    metrics_exporter_prometheus::BuildError,
);
