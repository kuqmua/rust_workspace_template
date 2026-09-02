#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::IntoInnerFrom,
)]
pub(crate) struct MetricsExporterPrometheusRenderer(metrics_exporter_prometheus::PrometheusHandle);
