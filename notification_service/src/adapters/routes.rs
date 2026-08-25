#![allow(clippy::single_call_fn)] // route endpoints and router composition each have one registry or runtime owner
#![allow(clippy::arbitrary_source_item_ordering)] // OpenAPI registry declarations stay beside their generated endpoint bindings
#![allow(clippy::needless_for_each)] // utoipa OpenApi derive expands to an internal for_each

#[frontend_contract::domain_types::route_openapi()]
async fn create_notification(
    state: crate::domain_types::AxumNotificationState,
    request: crate::domain_types::AxumNotificationJson,
) -> Result<
    crate::domain_types::AxumNotificationResponse,
    crate::domain_types::CreateNotificationError,
> {
    let id = uuid::Uuid::new_v4();
    let message = request.into_inner().into_message();
    let insert_sql = constants_str::VALUE_1A78C1E1;
    let _created = sqlx::query(insert_sql)
        .bind(id)
        .bind(message.as_ref())
        .execute(state.get().pool.as_ref())
        .await
        .map_err(|error| {
            crate::domain_types::CreateNotificationError::Persistence(
                server_runtime_http::domain_types::ObservedError::capture(
                    crate::domain_types::SqlxNotificationDatabaseError::from(error),
                    server_runtime_http::domain_types::ObservedErrorCode::from(
                        crate::domain_types::NotificationErrorCode::Persistence.get(),
                    ),
                ),
            )
        })?;
    Ok(crate::domain_types::AxumNotificationResponse::from(
        axum::response::IntoResponse::into_response((
            http::StatusCode::CREATED,
            axum::Json(
                notification_service_contract::domain_types::CreateNotificationRes::new(
                    notification_service_contract::domain_types::UuidNotificationId::from(id),
                ),
            ),
        )),
    ))
}

#[frontend_contract::domain_types::route_operation]
async fn metrics(
    state: crate::domain_types::AxumNotificationState,
) -> Result<server_runtime_http::domain_types::MetricsResponseBody, crate::domain_types::MetricsError>
{
    state.get().metrics.render().map_err(|error| {
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

#[frontend_contract::domain_types::route_operation]
async fn open_api() -> crate::domain_types::AxumNotificationResponse {
    let mut document = NotificationApiRouteRegistry::open_api();
    document.merge(utoipa::openapi::OpenApi::from(
        common_routes::domain_types::CommonRoutesOpenApi::open_api(),
    ));
    crate::domain_types::AxumNotificationResponse::from(
        axum::response::IntoResponse::into_response(axum::Json(document)),
    )
}

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[frontend_contract::domain_types::route_registry(
    state = crate::domain_types::NotificationState,
    family = notification_service_contract::domain_types::NotificationRouteFamily;
    ("", "");
    schemas(
        notification_service_contract::domain_types::NotificationMessage,
        notification_service_contract::domain_types::UuidNotificationId
    );
    (
        notification_service_contract::domain_types::CreateNotificationRoute,
        create_notification
    ),
)]
#[openapi()]
pub(super) struct NotificationApiRouteRegistry;

#[cfg(test)]
pub(crate) fn open_api_document() -> utoipa::openapi::OpenApi {
    NotificationApiRouteRegistry::open_api()
}

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[frontend_contract::domain_types::endpoint_registry(
    state = crate::domain_types::NotificationState;
    (
        notification_service_contract::domain_types::NotificationOperationalRoute::Metrics,
        metrics
    ),
    (
        notification_service_contract::domain_types::NotificationOperationalRoute::OpenApi,
        open_api
    ),
)]
struct NotificationRouteRegistry;

pub(crate) fn router(
    state: crate::domain_types::NotificationState,
    body_maximum_bytes: crate::domain_types::NotificationBodyMaximumBytes,
) -> crate::domain_types::AxumNotificationRouter {
    let common_routes = axum::Router::from(common_routes::adapters::common_routes(
        common_routes::domain_types::ArcCommonRoutesAppState::from(std::sync::Arc::new(
            state.clone(),
        )),
    ));
    crate::domain_types::AxumNotificationRouter::from(
        NotificationRouteRegistry::router()
            .merge(NotificationApiRouteRegistry::router())
            .layer(axum::extract::DefaultBodyLimit::max(
                body_maximum_bytes.get(),
            ))
            .with_state(state)
            .merge(common_routes),
    )
}
