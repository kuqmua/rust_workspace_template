#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn audit_query_log(
    auth: crate::admin_auth_req::AdminAuthReq,
    query: crate::axum_admin_query::AxumAdminQuery<crate::admin_audit_query::AdminAuditQuery>,
) -> Result<crate::axum_admin_response::AxumAdminResponse, crate::admin_error::AdminError> {
    if !query.0.cursor_is_complete().get() {
        return Err(crate::admin_error::AdminError::Validation);
    }
    let _actor = crate::authorization_authorize_generated_request::authorization_authorize_generated_request(
        auth.state.as_ref(),
        crate::http_admin_header_map_ref::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
        server_admin_contract::admin_permission::AdminPermission::AuditLogRead.as_str(),
        server_admin_core::std_admin_bool::StdAdminBool::from(false),
    )
    .await?;
    let page = crate::query_audit_log::query_audit_log(
        crate::sqlx_admin_repository_pool_ref::SqlxAdminRepositoryPoolRef::from(
            auth.state.as_ref().pool.as_ref(),
        ),
        query.0,
    )
    .await
    .map_err(|repository_error| match repository_error {
        crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue => {
            crate::admin_error::AdminError::Validation
        }
        crate::admin_repository_error::AdminRepositoryError::Sqlx(sqlx_error) => {
            crate::admin_error::AdminError::postgresql(sqlx_error)
        }
    })?;
    Ok(crate::json_response::json_response(page))
}
