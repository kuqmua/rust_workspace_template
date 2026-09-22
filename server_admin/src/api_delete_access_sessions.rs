#[proc_macro_frontend_contract_route_openapi::route_openapi(tag = "admin_tables")]
#[allow(
    clippy::single_call_fn,
    reason = "typed route registration requires a named endpoint function"
)]
pub(crate) async fn api_delete_access_sessions(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    axum_admin_json: crate::axum_admin_json::AxumAdminJson<
        server_admin_contract::admin_delete_access_sessions_request::AdminDeleteAccessSessionsRequest,
    >,
) -> Result<
    crate::axum_admin_response::AxumAdminResponse,
    crate::application_auth::AdminDeleteAccessSessionsError,
> {
    let select_filtered_access_session_ids = async |
        mut sqlx_admin_repository_connection_mut_ref: crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef<'_>,
        admin_access_session_filter: &server_admin_contract::admin_access_session_filter::AdminAccessSessionFilter,
    | -> Result<
        pg_crud_common::list_items::ListItems<crate::admin_session_id::AdminSessionId>,
        crate::admin_error::AdminError,
    > {
        if admin_access_session_filter.session_id().is_none()
            && admin_access_session_filter.user_id().is_none()
        {
            return Err(crate::admin_error::AdminError::Validation);
        }
        let session_id = match admin_access_session_filter.session_id() {
            Some(value) => match uuid::Uuid::parse_str(value.to_string().as_str()) {
                Ok(admin_session_uuid) => Some(admin_session_uuid),
                Err(..) => {
                    return Err(crate::map_repository_error::map_repository_error(
                        crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue,
                    ));
                }
            },
            None => None,
        };
        let matches = sqlx::query_scalar::<_, uuid::Uuid>(
            constants_str::SERVER_ADMIN_SELECT_FILTERED_ACCESS_SESSIONS_SQL,
        )
        .bind(session_id)
        .bind(
            admin_access_session_filter
                .user_id()
                .copied()
                .map(i64::from),
        )
        .fetch_all(&mut **sqlx_admin_repository_connection_mut_ref)
        .await
        .map_err(crate::admin_error::AdminError::from)?;
        if matches.is_empty() {
            return Err(crate::admin_error::AdminError::Conflict);
        }
        if matches.len() > 10_000 {
            return Err(crate::admin_error::AdminError::Validation);
        }
        Ok(pg_crud_common::list_items::ListItems::from(
            matches
                .into_iter()
                .map(server_admin_core::uuid_admin_value::UuidAdminValue::from)
                .map(crate::admin_session_id::AdminSessionId::from)
                .collect::<Vec<_>>(),
        ))
    };
    let actor = crate::authorize_custom::authorize_custom(
        &admin_auth_request,
        server_admin_contract::admin_rule::AdminRule::AccessSessionsDelete,
    )
    .await
    .map_err(crate::application_auth::AdminDeleteAccessSessionsError::from)?;
    let mut transaction = admin_auth_request
        .get_state()
        .as_ref()
        .get_pool()
        .as_ref()
        .begin()
        .await
        .map_err(crate::admin_error::AdminError::from)
        .map_err(crate::application_auth::AdminDeleteAccessSessionsError::from)?;
    let selected = Vec::from(
        select_filtered_access_session_ids(
            crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(
                &mut *transaction,
            ),
            axum_admin_json.into_inner().filter(),
        )
        .await
        .map_err(crate::application_auth::AdminDeleteAccessSessionsError::from)?,
    );
    let completed_transaction = futures::TryStreamExt::try_fold(
        futures::stream::iter(selected.into_iter().map(Ok::<_, crate::admin_error::AdminError>)),
        crate::sqlx_admin_transaction::SqlxAdminTransaction::from(transaction),
        async |mut sqlx_admin_transaction, admin_session_id| {
            sqlx::query(constants_str::SERVER_ADMIN_REVOKE_ADMIN_ACCESS_SESSION_SQL)
                .bind(admin_session_id.get().get())
                .execute(&mut **sqlx_admin_transaction)
                .await
                .map_err(crate::admin_error::AdminError::from)
                .map(drop)?;
            crate::record_audit_success_in_connection::record_audit_success_in_connection(
                crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(&mut **sqlx_admin_transaction),
                crate::admin_audit_success_ref::AdminAuditSuccessRef::new(
                    crate::admin_audit_action::AdminAuditAction::Delete,
                    actor.get_login(),
                    crate::admin_audit_resource::AdminAuditResource::Session,
                    crate::admin_audit_resource_id::AdminAuditResourceId::Session(admin_session_id),
                    actor.id(),
                ),
            ).await?;
            Ok(sqlx_admin_transaction)
        },
    ).await.map_err(crate::application_auth::AdminDeleteAccessSessionsError::from)?;
    sqlx::Transaction::from(completed_transaction)
        .commit()
        .await
        .map_err(crate::admin_error::AdminError::from)
        .map_err(crate::application_auth::AdminDeleteAccessSessionsError::from)?;
    Ok(crate::axum_admin_response::AxumAdminResponse::from(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
