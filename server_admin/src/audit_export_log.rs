#[allow(
    clippy::single_call_fn,
    reason = "audit export log remains a named owner because its boundary role is clearer and directly testable"
)]
pub(crate) async fn audit_export_log(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    axum_admin_query: crate::axum_admin_query::AxumAdminQuery<
        crate::admin_audit_query::AdminAuditQuery,
    >,
) -> Result<crate::axum_admin_response::AxumAdminResponse, crate::admin_error::AdminError> {
    if !axum_admin_query.get_inner().cursor_is_complete().get() {
        return Err(crate::admin_error::AdminError::Validation);
    }
    let actor = crate::authorization_authorize_generated_request::authorization_authorize_generated_request(
        admin_auth_request.get_state().as_ref(),
        crate::http_admin_header_map_ref::HttpAdminHeaderMapRef::from(admin_auth_request.get_headers().as_ref()),
        *admin_auth_request.get_peer(),
        server_admin_contract::admin_permission::AdminPermission::AuditLogExport.as_str(),
        server_admin_core::std_admin_bool::StdAdminBool::from(false),
    )
    .await?;
    let rate_subject = server_admin_core::std_admin_string::StdAdminString::try_from(
        actor.get_id().get().to_string(),
    )
    .map_err(|_error| crate::admin_error::AdminError::Validation)?;
    crate::enforce_rate_limit::enforce_rate_limit(
        admin_auth_request.get_state().as_ref(),
        crate::admin_rate_limit_scope::AdminRateLimitScope::AuditExport,
        &rate_subject,
        admin_auth_request
            .get_state()
            .as_ref()
            .get_policy()
            .get_audit_export_limit(),
        admin_auth_request
            .get_state()
            .as_ref()
            .get_policy()
            .get_audit_export_window(),
    )
    .await?;
    let page = crate::query_audit_log::query_audit_log(
        crate::sqlx_admin_repository_pool_ref::SqlxAdminRepositoryPoolRef::from(
            admin_auth_request.get_state().as_ref().get_pool().as_ref(),
        ),
        axum_admin_query.into_inner(),
    )
    .await
    .map_err(|error| match error {
        crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue => {
            crate::admin_error::AdminError::Validation
        }
        crate::admin_repository_error::AdminRepositoryError::Sqlx(sqlx_error) => {
            crate::admin_error::AdminError::postgresql(sqlx_error)
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
    let export = server_admin_contract::admin_audit_export::AdminAuditExport::new(
        server_admin_contract::admin_audit_export_csv::AdminAuditExportCsv::try_from(csv)
            .map_err(|_error| crate::admin_error::AdminError::Validation)?,
    );
    Ok(crate::axum_admin_response::AxumAdminResponse::from(
        axum::response::IntoResponse::into_response(axum::Json(export)),
    ))
}
