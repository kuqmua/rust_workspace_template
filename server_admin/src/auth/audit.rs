#![allow(clippy::single_call_fn)] // audit boundaries isolate append/query SQL from route and transaction facades
pub(super) async fn record_success_in_connection(
    mut connection: super::SqlxAdminPgConnectionRef<'_>,
    event: super::AdminAuditSuccessRef<'_>,
) -> Result<(), super::AdminApiError> {
    let details = server_admin_contract::SerdeJsonAdminAuditDetails::try_from(
        serde_json::json!({ "operation": event.action.as_str().as_ref(), "target_id": event.resource_id.value().as_ref() }),
    )
    .map_err(|_error| super::AdminApiError::Validation)?;
    let resource_id = event.resource_id.value();
    super::super::repository::audit::insert_audit_success(
        super::super::repository::SqlxAdminRepositoryConnectionMutRef::from(connection.as_mut()),
        event.user_id,
        event.login,
        event.action,
        event.resource,
        &resource_id,
        super::super::UuidAdminValue::from(uuid::Uuid::new_v4()),
        &details,
    )
    .await
    .map_err(super::AdminApiError::Pg)
}
pub(super) async fn query_log(
    auth: super::AdminAuthReq,
    query: super::AxumAdminQuery<super::AdminAuditQuery>,
) -> Result<super::AxumAdminResponse, super::AdminApiError> {
    let actor = super::authorize_generated_request(
        auth.state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
        super::super::AdminPermission::AuditLogRead.as_str(),
        super::super::StdAdminBool::from(false),
    )
    .await?;
    let rate_subject = super::super::StdAdminString::try_from(actor.id.0.to_string())
        .map_err(|_error| super::AdminApiError::Validation)?;
    super::rate_limit::enforce_rate_limit(
        auth.state.as_ref(),
        super::rate_limit::AdminRateLimitScope::AuditRead,
        &rate_subject,
        auth.state.as_ref().policy.audit_limit,
        auth.state.as_ref().policy.audit_window,
    )
    .await?;
    let views = super::super::repository::audit::query_audit_log(
        super::super::repository::SqlxAdminRepositoryPoolRef::from(
            auth.state.as_ref().pool.as_ref(),
        ),
        query.0,
    )
    .await
    .map_err(|repository_error| match repository_error {
        super::super::repository::AdminRepositoryError::InvalidStoredValue => {
            super::AdminApiError::Validation
        }
        super::super::repository::AdminRepositoryError::Sqlx(sqlx_error) => {
            super::AdminApiError::Pg(sqlx_error)
        }
    })?;
    Ok(super::AxumAdminResponse(
        axum::response::IntoResponse::into_response(axum::Json(views)),
    ))
}
