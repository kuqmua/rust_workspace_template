proc_macro_frontend_contract_route_registry::route_registry! {
    pub(crate);
    state = crate::metrics_exporter_prometheus_renderer::MetricsExporterPrometheusRenderer;
    (
        server_admin_contract::admin_frontend_path::AdminFrontendPath::Metrics,
        crate::admin_metrics_page::admin_metrics_page
    ),
}
