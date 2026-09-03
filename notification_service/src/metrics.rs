#[allow(
    clippy::single_call_fn,
    reason = "metrics remains a named owner because its boundary role is clearer and directly testable"
)]
#[proc_macro_frontend_contract_route_operation::route_operation]
pub(super) async fn metrics(
    notification_axum_state: crate::notification_axum_state::NotificationAxumState,
) -> Result<
    server_runtime_http::metrics_response_body::MetricsResponseBody,
    crate::metrics_error::MetricsError,
> {
    notification_axum_state
        .get()
        .get_metrics()
        .render()
        .map_err(|error| {
            crate::metrics_error::MetricsError::Render(
                server_observability::observed_error::ObservedError::capture(
                    error,
                    server_observability::observed_error_code::ObservedErrorCode::from(
                        crate::notification_error_code::NotificationErrorCode::MetricsRender.get(),
                    ),
                ),
            )
        })
}
