#[allow(clippy::single_call_fn, reason = "lint suppression is required here")]
#[proc_macro_frontend_contract::route_openapi()]
pub(super) async fn create_notification(
    notification_axum_state: crate::notification_axum_state::NotificationAxumState,
    notification_axum_json: crate::notification_axum_json::NotificationAxumJson,
) -> Result<
    crate::axum_notification_response::AxumNotificationResponse,
    crate::create_notification_error::CreateNotificationError,
> {
    let id = uuid::Uuid::new_v4();
    let message = notification_axum_json.into_inner().into_message();
    let insert_sql = constants_str::VALUE_1A78C1E1;
    let _created = sqlx::query(insert_sql)
        .bind(id)
        .bind(message.as_ref())
        .execute(notification_axum_state.get().get_pool().as_ref())
        .await
        .map_err(|error| {
            crate::create_notification_error::CreateNotificationError::Persistence(
                server_observability::observed_error::ObservedError::capture(
                    crate::sqlx_notification_database_error::SqlxNotificationDatabaseError::from(
                        error,
                    ),
                    server_observability::observed_error_code::ObservedErrorCode::from(
                        crate::notification_error_code::NotificationErrorCode::Persistence.get(),
                    ),
                ),
            )
        })?;
    Ok(
        crate::axum_notification_response::AxumNotificationResponse::from(
            axum::response::IntoResponse::into_response((
                http::StatusCode::CREATED,
                axum::Json(
                    notification_service_contract::create_notification_response::CreateNotificationResponse::new(
                        notification_service_contract::uuid_notification_id::UuidNotificationId::from(id),
                    ),
                ),
            )),
        ),
    )
}
