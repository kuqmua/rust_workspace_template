#![allow(clippy::single_call_fn)] // route endpoints are registered indirectly by axum
pub(super) async fn audit_query_log(
    auth: super::AdminAuthReq,
    query: super::AxumAdminQuery<super::AdminAuditQuery>,
) -> Result<super::AxumAdminResponse, super::AdminError> {
    if !query.0.cursor_is_complete().get() {
        return Err(super::AdminError::Validation);
    }
    let _actor = super::authorization_authorize_generated_request::authorization_authorize_generated_request(
        auth.state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
        super::super::AdminPermission::AuditLogRead.as_str(),
        super::super::StdAdminBool::from(false),
    )
    .await?;
    let page = crate::repository::query_audit_log::query_audit_log(
        crate::repository::SqlxAdminRepositoryPoolRef::from(auth.state.as_ref().pool.as_ref()),
        query.0,
    )
    .await
    .map_err(|repository_error| match repository_error {
        crate::repository::AdminRepositoryError::InvalidStoredValue => {
            super::AdminError::Validation
        }
        crate::repository::AdminRepositoryError::Sqlx(sqlx_error) => {
            super::AdminError::postgresql(sqlx_error)
        }
    })?;
    Ok(super::shared::json_response::json_response(page))
}
