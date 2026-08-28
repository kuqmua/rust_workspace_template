// The owner module retains lint-sensitive semantics from the original implementation.

#[allow(clippy::single_call_fn)] // operational route registry owns this OpenAPI endpoint handler
#[frontend_contract::domain_types::route_operation]
pub(super) async fn notification_open_api() -> crate::domain_types::AxumNotificationResponse {
    let mut document =
        super::notification_api_route_registry::NotificationApiRouteRegistry::open_api();
    document.merge(utoipa::openapi::OpenApi::from(
        common_routes::domain_types::CommonRoutesOpenApi::open_api(),
    ));
    crate::domain_types::AxumNotificationResponse::from(
        axum::response::IntoResponse::into_response(axum::Json(document)),
    )
}
