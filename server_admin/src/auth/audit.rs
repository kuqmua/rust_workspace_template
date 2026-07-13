#![allow(clippy::single_call_fn)] // audit boundaries isolate append/query SQL from route and transaction facades
pub(super) async fn record_success_in_connection(
    mut connection: super::SqlxAdminPgConnectionRef<'_>,
    event: super::AdminAuditSuccessRef<'_>,
) -> Result<(), super::AdminApiEr> {
    let details = serde_json::json!({ "operation": event.action.as_str().as_ref(), "target_id": event.resource_id.value().as_ref() });
    let _result = sqlx::query(
        "INSERT INTO admin_audit_log (user_id, user_login, action, resource, resource_id, request_id, succeeded, details) VALUES ($1, $2, $3, $4, $5, $6, TRUE, $7)",
    )
    .bind(event.user_id.0)
    .bind(event.login.as_ref())
    .bind(event.action.as_str().as_ref())
    .bind(event.resource.as_str().as_ref())
    .bind(event.resource_id.value().as_ref())
    .bind(uuid::Uuid::new_v4())
    .bind(details)
    .execute(connection.as_mut())
    .await
    .map_err(|er| super::AdminApiEr::Pg(super::super::SqlxAdminEr::from(er)))?;
    Ok(())
}
pub(super) async fn query_log(
    auth: super::AdminAuthReq,
    query: super::AxumAdminQuery<super::AdminAuditQuery>,
) -> Result<super::AxumAdminResponse, super::AdminApiEr> {
    let actor = super::authorize_generated_request(
        auth.state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        super::super::AdminPermission::AuditLogRead.as_str(),
        super::super::StdAdminBool::from(false),
    )
    .await?;
    let rate_subject = super::super::StdAdminString::try_from(actor.id.0.to_string())
        .map_err(|_er| super::AdminApiEr::Validation)?;
    super::rate_limit::enforce_rate_limit(
        auth.state.as_ref(),
        super::rate_limit::AdminRateLimitScope::AuditRead,
        &rate_subject,
        super::rate_limit::StdAdminRateLimitCount::from(60i64),
        super::rate_limit::StdAdminRateLimitWindowSeconds::from(60i32),
    )
    .await?;
    let action = query.0.action.map(super::super::AdminAuditAction::as_str);
    let resource = query
        .0
        .resource
        .map(super::super::AdminAuditResource::as_str);
    let rows = sqlx::query_as::<
        _,
        (
            i64,
            Option<i64>,
            Option<String>,
            String,
            String,
            Option<String>,
            bool,
            Option<serde_json::Value>,
            String,
        ),
    >(
        "SELECT id, user_id, user_login, action, resource, resource_id, succeeded, details, created_at::TEXT FROM admin_audit_log WHERE ($1::BIGINT IS NULL OR user_id = $1) AND ($2::TEXT IS NULL OR action = $2) AND ($3::TEXT IS NULL OR resource = $3) AND ($4::TIMESTAMPTZ IS NULL OR created_at >= $4::TIMESTAMPTZ) AND ($5::TIMESTAMPTZ IS NULL OR created_at <= $5::TIMESTAMPTZ) ORDER BY created_at DESC LIMIT 200",
    )
    .bind(query.0.user_id.map(|user_id| user_id.0))
    .bind(action.map(|value| value.as_ref().to_owned()))
    .bind(resource.map(|value| value.as_ref().to_owned()))
    .bind(query.0.created_after.as_ref().map(|value| value.as_ref().as_str()))
    .bind(query.0.created_before.as_ref().map(|value| value.as_ref().as_str()))
    .fetch_all(auth.state.as_ref().pool.as_ref())
    .await
    .map_err(|er| super::AdminApiEr::Pg(super::super::SqlxAdminEr::from(er)))?;
    let views = rows
        .into_iter()
        .map(|row| {
            Ok(server_admin_contract::AdminAuditView::new(
                server_admin_contract::AdminText::try_from(row.3)
                    .map_err(|_er| super::AdminApiEr::Validation)?,
                server_admin_contract::AdminAuditTimestamp::try_from(row.8)
                    .map_err(|_er| super::AdminApiEr::Validation)?,
                row.7
                    .map(server_admin_contract::SerdeJsonAdminAuditDetails::from),
                server_admin_contract::AdminAuditLogId::from(row.0),
                server_admin_contract::AdminText::try_from(row.4)
                    .map_err(|_er| super::AdminApiEr::Validation)?,
                row.5
                    .map(server_admin_contract::AdminText::try_from)
                    .transpose()
                    .map_err(|_er| super::AdminApiEr::Validation)?,
                server_admin_contract::AdminBool::from(row.6),
                row.1.map(server_admin_contract::AdminUserId::from),
                row.2
                    .map(server_admin_contract::AdminLogin::try_from)
                    .transpose()
                    .map_err(|_er| super::AdminApiEr::Validation)?,
            ))
        })
        .collect::<Result<Vec<server_admin_contract::AdminAuditView>, super::AdminApiEr>>()?;
    Ok(super::AxumAdminResponse(
        axum::response::IntoResponse::into_response(axum::Json(views)),
    ))
}
