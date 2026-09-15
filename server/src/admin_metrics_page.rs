#[allow(
    clippy::single_call_fn,
    reason = "the generated route registry requires a named administrator metrics page handler"
)]
#[proc_macro_frontend_contract_route_operation::route_operation]
pub(crate) async fn admin_metrics_page(
    axum_metrics_exporter_prometheus_renderer: crate::axum_metrics_exporter_prometheus_renderer::AxumMetricsExporterPrometheusRenderer,
) -> axum::response::Response {
    server_runtime_http::metrics_response_body::MetricsResponseBody::try_from(
        metrics_exporter_prometheus::PrometheusHandle::from(
            crate::metrics_exporter_prometheus_renderer::MetricsExporterPrometheusRenderer::from(
                axum_metrics_exporter_prometheus_renderer,
            ),
        )
        .render(),
    )
    .map_or_else(
        |_error| {
            axum::response::IntoResponse::into_response(
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            )
        },
        |metrics_response_body| {
            let title_result = frontend_admin::admin_ssr_text::AdminSsrText::try_from(
                constants_str::METRICS_ALT.to_owned(),
            );
            let text_result = frontend_admin::admin_ssr_text::AdminSsrText::try_from(
                metrics_response_body.into_inner(),
            );
            match (title_result, text_result) {
                (Ok(title), Ok(text)) => {
                    axum::response::IntoResponse::into_response(axum::response::Html(String::from(
                        frontend_admin::render_text_page::render_text_page(
                            server_admin_contract::admin_page::AdminPage::Metrics,
                            title,
                            text,
                        ),
                    )))
                }
                (Err(_error), _) | (_, Err(_error)) => axum::response::IntoResponse::into_response(
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                ),
            }
        },
    )
}
