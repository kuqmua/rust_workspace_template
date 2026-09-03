#[allow(clippy::single_call_fn, reason = "lint suppression is required here")]
pub(crate) async fn audit_query_log(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    axum_admin_query: crate::axum_admin_query::AxumAdminQuery<
        crate::admin_audit_query::AdminAuditQuery,
    >,
) -> Result<crate::axum_admin_response::AxumAdminResponse, crate::admin_error::AdminError> {
    if !axum_admin_query.get_inner().cursor_is_complete().get() {
        return Err(crate::admin_error::AdminError::Validation);
    }
    let _actor = crate::authorization_authorize_generated_request::authorization_authorize_generated_request(
        admin_auth_request.get_state().as_ref(),
        crate::http_admin_header_map_ref::HttpAdminHeaderMapRef::from(admin_auth_request.get_headers().as_ref()),
        *admin_auth_request.get_peer(),
        server_admin_contract::admin_permission::AdminPermission::AuditLogRead.as_str(),
        server_admin_core::std_admin_bool::StdAdminBool::from(false),
    )
    .await?;
    let page = crate::query_audit_log::query_audit_log(
        crate::sqlx_admin_repository_pool_ref::SqlxAdminRepositoryPoolRef::from(
            admin_auth_request.get_state().as_ref().get_pool().as_ref(),
        ),
        axum_admin_query.into_inner(),
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
