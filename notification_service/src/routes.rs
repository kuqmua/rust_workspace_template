#[frontend_contract::route_openapi()]
async fn create_notification(
    state: super::AxumNotificationState,
    request: super::AxumNotificationJson,
) -> Result<super::AxumNotificationResponse, super::CreateNotificationError> {
    let id = uuid::Uuid::new_v4();
    let message = request.0.into_message();
    let insert_sql = "INSERT INTO notifications (id, message) VALUES ($1, $2)";
    let _created = sqlx::query(insert_sql)
        .bind(id)
        .bind(message.as_ref())
        .execute(state.0.pool.as_ref())
        .await
        .map_err(|error| {
            super::CreateNotificationError::Persistence(
                server_runtime_http::ObservedError::capture(
                    super::SqlxNotificationDatabaseError::from(error),
                    server_runtime_http::ObservedErrorCode::from(
                        super::NotificationErrorCode::Persistence.get(),
                    ),
                ),
            )
        })?;
    Ok(super::AxumNotificationResponse::from(
        axum::response::IntoResponse::into_response((
            http::StatusCode::CREATED,
            axum::Json(notification_service_contract::CreateNotificationRes::new(
                notification_service_contract::UuidNotificationId::from(id),
            )),
        )),
    ))
}

#[frontend_contract::route_operation]
async fn metrics(
    state: super::AxumNotificationState,
) -> Result<server_runtime_http::MetricsResponseBody, super::MetricsError> {
    server_runtime_http::MetricsResponseBody::try_from(state.0.metrics.0.render()).map_err(
        |error| {
            super::MetricsError::Render(server_runtime_http::ObservedError::capture(
                error,
                server_runtime_http::ObservedErrorCode::from(
                    super::NotificationErrorCode::MetricsRender.get(),
                ),
            ))
        },
    )
}

#[frontend_contract::route_operation]
async fn open_api() -> super::AxumNotificationResponse {
    let mut document = NotificationApiRouteRegistry::open_api();
    document.merge(utoipa::openapi::OpenApi::from(
        common_routes::CommonRoutesOpenApi::open_api(),
    ));
    super::AxumNotificationResponse::from(axum::response::IntoResponse::into_response(axum::Json(
        document,
    )))
}

#[frontend_contract::route_registry(
    state = super::NotificationState,
    family = notification_service_contract::NotificationRouteFamily;
    ("", "");
    schemas(
        notification_service_contract::NotificationMessage,
        notification_service_contract::UuidNotificationId
    );
    (
        notification_service_contract::CreateNotificationRoute,
        create_notification
    ),
)]
#[openapi()]
pub(super) struct NotificationApiRouteRegistry;

#[cfg(test)]
pub(super) fn open_api_document() -> utoipa::openapi::OpenApi {
    NotificationApiRouteRegistry::open_api()
}

#[frontend_contract::handler_registry(
    state = super::NotificationState;
    (
        notification_service_contract::NotificationOperationalRoute::Metrics,
        metrics
    ),
    (
        notification_service_contract::NotificationOperationalRoute::OpenApi,
        open_api
    ),
)]
struct NotificationRouteRegistry;

pub(super) fn router(
    state: super::NotificationState,
    body_maximum_bytes: super::NotificationBodyMaximumBytes,
) -> super::AxumNotificationRouter {
    let common_routes = axum::Router::from(common_routes::common_routes(
        common_routes::StdArcCommonRoutesAppState::from(std::sync::Arc::new(state.clone())),
    ));
    super::AxumNotificationRouter::from(
        NotificationRouteRegistry::router()
            .merge(NotificationApiRouteRegistry::router())
            .layer(axum::extract::DefaultBodyLimit::max(body_maximum_bytes.0))
            .with_state(state)
            .merge(common_routes),
    )
}
