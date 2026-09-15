proc_macro_frontend_contract_route_registry::route_registry! {
    pub(crate);
    state = crate::metrics_exporter_prometheus_renderer::MetricsExporterPrometheusRenderer;
    (
        server_admin_contract::admin_route::AdminRoute::Metrics,
        crate::admin_metrics::admin_metrics
    ),
}
