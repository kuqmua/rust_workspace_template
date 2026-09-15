#[allow(
    clippy::single_call_fn,
    reason = "the generated route registry requires a named administrator metrics handler"
)]
#[proc_macro_frontend_contract_route_operation::route_operation]
pub(crate) async fn admin_metrics(
    axum_metrics_exporter_prometheus_renderer: crate::axum_metrics_exporter_prometheus_renderer::AxumMetricsExporterPrometheusRenderer,
) -> Result<axum::response::Response, crate::admin_metrics_error::AdminMetricsError> {
    server_runtime_http::metrics_response_body::MetricsResponseBody::try_from(
        metrics_exporter_prometheus::PrometheusHandle::from(
            crate::metrics_exporter_prometheus_renderer::MetricsExporterPrometheusRenderer::from(
                axum_metrics_exporter_prometheus_renderer,
            ),
        )
        .render(),
    )
    .map(|metrics_response_body| {
        axum::response::IntoResponse::into_response((
            axum::http::StatusCode::OK,
            metrics_response_body.into_inner(),
        ))
    })
    .map_err(crate::admin_metrics_error::AdminMetricsError::Render)
}
