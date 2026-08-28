// The owner module retains lint-sensitive semantics from the original implementation.

#[allow(clippy::single_call_fn)] // typed route registry owns this endpoint handler
#[frontend_contract::route_openapi()]
pub(super) async fn create_notification(
    state: crate::AxumNotificationState,
    request: crate::AxumNotificationJson,
) -> Result<crate::AxumNotificationResponse, crate::CreateNotificationError> {
    let id = uuid::Uuid::new_v4();
    let message = request.into_inner().into_message();
    let insert_sql = constants_str::VALUE_1A78C1E1;
    let _created = sqlx::query(insert_sql)
        .bind(id)
        .bind(message.as_ref())
        .execute(state.get().get_pool().as_ref())
        .await
        .map_err(|error| {
            crate::CreateNotificationError::Persistence(
                server_runtime_http::domain_types::ObservedError::capture(
                    crate::SqlxNotificationDatabaseError::from(error),
                    server_runtime_http::domain_types::ObservedErrorCode::from(
                        crate::NotificationErrorCode::Persistence.get(),
                    ),
                ),
            )
        })?;
    Ok(crate::AxumNotificationResponse::from(
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
