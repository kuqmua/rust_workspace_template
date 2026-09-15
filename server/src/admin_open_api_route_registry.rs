proc_macro_frontend_contract_route_registry::route_registry! {
    pub(crate);
    state = crate::metrics_exporter_prometheus_renderer::MetricsExporterPrometheusRenderer;
    (
        server_admin_contract::admin_route::AdminRoute::OpenApi,
        crate::admin_open_api::admin_open_api
    ),
}
