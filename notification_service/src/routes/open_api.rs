#![allow(clippy::single_call_fn)]

#[frontend_contract::domain_types::route_operation]
pub(super) async fn open_api() -> crate::domain_types::AxumNotificationResponse {
    let mut document =
        super::notification_api_route_registry::NotificationApiRouteRegistry::open_api();
    document.merge(utoipa::openapi::OpenApi::from(
        common_routes::domain_types::CommonRoutesOpenApi::open_api(),
    ));
    crate::domain_types::AxumNotificationResponse::from(
        axum::response::IntoResponse::into_response(axum::Json(document)),
    )
}
