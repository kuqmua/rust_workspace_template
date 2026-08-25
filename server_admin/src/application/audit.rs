#![allow(clippy::single_call_fn)] // audit boundaries isolate append/query SQL from route and transaction facades
pub(super) async fn record_success_in_connection(
    mut connection: super::SqlxAdminPgConnectionRef<'_>,
    event: super::AdminAuditSuccessRef<'_>,
) -> Result<(), super::AdminError> {
    let details = server_admin_contract::domain_types::SerdeJsonAdminAuditDetails::try_from(
        serde_json::json!({ "operation": event.action.as_str().as_ref(), "target_id": event.resource_id.value().as_ref() }),
    )
    .map_err(|_error| super::AdminError::Validation)?;
    let resource_id = event.resource_id.value();
    crate::adapters::repository::audit::insert_audit_success(
        crate::adapters::repository::SqlxAdminRepositoryConnectionMutRef::from(connection.as_mut()),
        event.user_id,
        event.login,
        event.action,
        event.resource,
        &resource_id,
        super::super::UuidAdminValue::from(uuid::Uuid::new_v4()),
        &details,
    )
    .await
    .map_err(super::AdminError::pg)
}
pub(super) async fn query_page(
    auth: super::AdminAuthReq,
    query: super::AxumAdminQuery<super::AdminAuditQuery>,
) -> Result<server_admin_contract::domain_types::AdminAuditPage, super::AdminError> {
    if !query.0.cursor_is_complete().get() {
        return Err(super::AdminError::Validation);
    }
    let _actor = super::authorize_generated_request(
        auth.state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
        super::super::AdminPermission::AuditLogRead.as_str(),
        super::super::StdAdminBool::from(false),
    )
    .await?;
    let page = crate::adapters::repository::audit::query_audit_log(
        crate::adapters::repository::SqlxAdminRepositoryPoolRef::from(
            auth.state.as_ref().pool.as_ref(),
        ),
        query.0,
    )
    .await
    .map_err(|repository_error| match repository_error {
        crate::adapters::repository::AdminRepositoryError::InvalidStoredValue => {
            super::AdminError::Validation
        }
        crate::adapters::repository::AdminRepositoryError::Sqlx(sqlx_error) => {
            super::AdminError::pg(sqlx_error)
        }
    })?;
    Ok(page)
}
pub(super) async fn query_log(
    auth: super::AdminAuthReq,
    query: super::AxumAdminQuery<super::AdminAuditQuery>,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    query_page(auth, query)
        .await
        .map(super::shared::json_response)
}
pub(super) async fn export_log(
    auth: super::AdminAuthReq,
    query: super::AxumAdminQuery<super::AdminAuditQuery>,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    if !query.0.cursor_is_complete().get() {
        return Err(super::AdminError::Validation);
    }
    let actor = super::authorize_generated_request(
        auth.state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
        super::super::AdminPermission::AuditLogExport.as_str(),
        super::super::StdAdminBool::from(false),
    )
    .await?;
    let rate_subject = super::super::StdAdminString::try_from(actor.id.get().to_string())
        .map_err(|_error| super::AdminError::Validation)?;
    super::rate_limit::enforce_rate_limit(
        auth.state.as_ref(),
        super::rate_limit::AdminRateLimitScope::AuditExport,
        &rate_subject,
        auth.state.as_ref().policy.audit_export_limit,
        auth.state.as_ref().policy.audit_export_window,
    )
    .await?;
    let page = crate::adapters::repository::audit::query_audit_log(
        crate::adapters::repository::SqlxAdminRepositoryPoolRef::from(
            auth.state.as_ref().pool.as_ref(),
        ),
        query.0,
    )
    .await
    .map_err(|error| match error {
        crate::adapters::repository::AdminRepositoryError::InvalidStoredValue => {
            super::AdminError::Validation
        }
        crate::adapters::repository::AdminRepositoryError::Sqlx(sqlx_error) => {
            super::AdminError::pg(sqlx_error)
        }
    })?;
    let mut csv = String::from(constants_str::AUDIT_CSV_HEADER);
    page.items().iter().for_each(|value| {
        let fields = [
            value.id().to_string(),
            value.created_at().to_string(),
            value.user_id().map(|id| id.to_string()).unwrap_or_default(),
            value
                .user_login()
                .map(ToString::to_string)
                .unwrap_or_default(),
            value.action().to_string(),
            value.resource().to_string(),
            value
                .resource_id()
                .map(ToString::to_string)
                .unwrap_or_default(),
            value.succeeded().to_string(),
            value.details().map(ToString::to_string).unwrap_or_default(),
        ];
        csv.push_str(
            fields
                .map(|field| format!("\"{}\"", field.replace('"', "\"\"")))
                .join(constants_str::TEXT_ALT_7)
                .as_str(),
        );
        csv.push('\n');
    });
    let export = server_admin_contract::domain_types::AdminAuditExport::new(
        server_admin_contract::domain_types::AdminAuditExportCsv::try_from(csv)
            .map_err(|_error| super::AdminError::Validation)?,
    );
    Ok(super::AxumAdminResponse(
        axum::response::IntoResponse::into_response(axum::Json(export)),
    ))
}
