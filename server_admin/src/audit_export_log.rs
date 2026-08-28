#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn audit_export_log(
    auth: crate::AdminAuthReq,
    query: crate::AxumAdminQuery<crate::AdminAuditQuery>,
) -> Result<crate::AxumAdminResponse, crate::AdminError> {
    if !query.0.cursor_is_complete().get() {
        return Err(crate::AdminError::Validation);
    }
    let actor = crate::authorization_authorize_generated_request::authorization_authorize_generated_request(
        auth.state.as_ref(),
        crate::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
        crate::AdminPermission::AuditLogExport.as_str(),
        crate::StdAdminBool::from(false),
    )
    .await?;
    let rate_subject = crate::StdAdminString::try_from(actor.id.get().to_string())
        .map_err(|_error| crate::AdminError::Validation)?;
    crate::rate_limit::enforce_rate_limit(
        auth.state.as_ref(),
        crate::rate_limit::AdminRateLimitScope::AuditExport,
        &rate_subject,
        auth.state.as_ref().policy.audit_export_limit,
        auth.state.as_ref().policy.audit_export_window,
    )
    .await?;
    let page = crate::repository::query_audit_log::query_audit_log(
        crate::repository::SqlxAdminRepositoryPoolRef::from(auth.state.as_ref().pool.as_ref()),
        query.0,
    )
    .await
    .map_err(|error| match error {
        crate::repository::AdminRepositoryError::InvalidStoredValue => {
            crate::AdminError::Validation
        }
        crate::repository::AdminRepositoryError::Sqlx(sqlx_error) => {
            crate::AdminError::postgresql(sqlx_error)
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
            .map_err(|_error| crate::AdminError::Validation)?,
    );
    Ok(crate::AxumAdminResponse(
        axum::response::IntoResponse::into_response(axum::Json(export)),
    ))
}
