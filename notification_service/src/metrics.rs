// The owner module retains lint-sensitive semantics from the original implementation.
#![allow(clippy::single_call_fn)]

#[frontend_contract::domain_types::route_operation]
pub(super) async fn metrics(
    state: crate::domain_types::AxumNotificationState,
) -> Result<server_runtime_http::domain_types::MetricsResponseBody, crate::domain_types::MetricsError>
{
    state.get().get_metrics().render().map_err(|error| {
        crate::domain_types::MetricsError::Render(
            server_runtime_http::domain_types::ObservedError::capture(
                error,
                server_runtime_http::domain_types::ObservedErrorCode::from(
                    crate::domain_types::NotificationErrorCode::MetricsRender.get(),
                ),
            ),
        )
    })
}
