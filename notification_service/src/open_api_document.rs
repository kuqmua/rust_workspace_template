pub(crate) fn open_api_document() -> utoipa::openapi::OpenApi {
    super::notification_api_route_registry::NotificationApiRouteRegistry::open_api()
}
