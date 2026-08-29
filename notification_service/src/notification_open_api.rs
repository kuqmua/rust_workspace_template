// The owner module retains lint-sensitive semantics from the original implementation.

#[allow(clippy::single_call_fn)] // operational route registry owns this OpenAPI endpoint handler
#[frontend_contract_macros::route_operation]
pub(super) async fn notification_open_api()
-> crate::axum_notification_response::AxumNotificationResponse {
    let mut document =
        super::notification_api_route_registry::NotificationApiRouteRegistry::open_api();
    document.merge(utoipa::openapi::OpenApi::from(
        common_routes::common_routes_open_api::CommonRoutesOpenApi::open_api(),
    ));
    crate::axum_notification_response::AxumNotificationResponse::from(
        axum::response::IntoResponse::into_response(axum::Json(document)),
    )
}
