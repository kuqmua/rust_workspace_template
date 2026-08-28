// The owner module retains lint-sensitive semantics from the original implementation.

#[allow(clippy::single_call_fn)] // operational route registry owns this endpoint handler
#[frontend_contract::route_operation]
pub(super) async fn metrics(
    state: crate::AxumNotificationState,
) -> Result<server_runtime_http::domain_types::MetricsResponseBody, crate::MetricsError> {
    state.get().get_metrics().render().map_err(|error| {
        crate::MetricsError::Render(server_runtime_http::domain_types::ObservedError::capture(
            error,
            server_runtime_http::domain_types::ObservedErrorCode::from(
                crate::NotificationErrorCode::MetricsRender.get(),
            ),
        ))
    })
}
