#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn audit_query_log(
    auth: crate::AdminAuthReq,
    query: crate::AxumAdminQuery<crate::AdminAuditQuery>,
) -> Result<crate::AxumAdminResponse, crate::AdminError> {
    if !query.0.cursor_is_complete().get() {
        return Err(crate::AdminError::Validation);
    }
    let _actor = crate::authorization_authorize_generated_request::authorization_authorize_generated_request(
        auth.state.as_ref(),
        crate::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
        crate::AdminPermission::AuditLogRead.as_str(),
        crate::StdAdminBool::from(false),
    )
    .await?;
    let page = crate::repository::query_audit_log::query_audit_log(
        crate::repository::SqlxAdminRepositoryPoolRef::from(auth.state.as_ref().pool.as_ref()),
        query.0,
    )
    .await
    .map_err(|repository_error| match repository_error {
        crate::repository::AdminRepositoryError::InvalidStoredValue => {
            crate::AdminError::Validation
        }
        crate::repository::AdminRepositoryError::Sqlx(sqlx_error) => {
            crate::AdminError::postgresql(sqlx_error)
        }
    })?;
    Ok(crate::shared::json_response::json_response(page))
}
