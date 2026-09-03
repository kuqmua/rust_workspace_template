#[allow(clippy::single_call_fn, reason = "lint suppression is required here")]
#[proc_macro_frontend_contract::route_operation]
pub(super) async fn notification_open_api()
-> crate::axum_notification_response::AxumNotificationResponse {
    let mut document = super::notification_api_route_registry::open_api();
    document.merge(utoipa::openapi::OpenApi::from(
        common_routes::common_routes_open_api::CommonRoutesOpenApi::open_api(),
    ));
    crate::axum_notification_response::AxumNotificationResponse::from(
        axum::response::IntoResponse::into_response(axum::Json(document)),
    )
}
