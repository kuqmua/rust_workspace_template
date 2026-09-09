#[proc_macro_frontend_contract_route_openapi::route_openapi(tag = "admin_users")]
#[allow(
    clippy::single_call_fn,
    reason = "typed route registration requires a named endpoint function"
)]
pub(crate) async fn api_update_users(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    axum_admin_json: crate::axum_admin_json::AxumAdminJson<
        server_admin_contract::admin_update_users_request::AdminUpdateUsersRequest,
    >,
) -> Result<
    crate::axum_admin_response::AxumAdminResponse,
    crate::application_auth::AdminUpdateUsersError,
> {
    let update_users = async {
        let actor = crate::authorize_custom::authorize_custom(
            &admin_auth_request,
            server_admin_contract::admin_permission::AdminPermission::UsersUpdate,
        )
        .await?;
        let request = axum_admin_json.into_inner();
        let updates = request.updates().as_ref();
        let mut identifiers = std::collections::BTreeSet::new();
        if updates.is_empty()
            || updates
                .iter()
                .any(|update| !identifiers.insert(i64::from(update.user_id())))
        {
            return Err(crate::admin_error::AdminError::Validation);
        }
        let mut ordered = updates.iter().collect::<Vec<_>>();
        ordered.sort_by_key(|update| {
            (
                update
                    .changes()
                    .is_banned()
                    .copied()
                    .is_some_and(bool::from),
                i64::from(update.user_id()),
            )
        });
        let mut transaction = admin_auth_request
            .get_state()
            .as_ref()
            .get_pool()
            .as_ref()
            .begin()
            .await
            .map_err(crate::admin_error::AdminError::from)?;
        if updates
            .iter()
            .any(|update| update.changes().is_banned().is_some())
        {
            crate::lock_last_admin::lock_last_admin(
                crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(&mut *transaction),
            ).await?;
        }
        let completed_transaction = futures::TryStreamExt::try_fold(
            futures::stream::iter(ordered.into_iter().map(Ok::<_, crate::admin_error::AdminError>)),
            crate::sqlx_admin_transaction::SqlxAdminTransaction::from(transaction),
            async |mut sqlx_admin_transaction, admin_user_update| {
                crate::apply_user_update_in_connection::apply_user_update_in_connection(
                    crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(&mut **sqlx_admin_transaction),
                    &actor,
                    server_admin_core::admin_user_record_id::AdminUserRecordId::from(admin_user_update.user_id().value()),
                    admin_user_update.changes(),
                ).await?;
                Ok(sqlx_admin_transaction)
            },
        ).await?;
        sqlx::Transaction::from(completed_transaction)
            .commit()
            .await
            .map_err(crate::admin_error::AdminError::from)?;
        Ok(crate::axum_admin_response::AxumAdminResponse::from(
            axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
        ))
    };
    update_users
        .await
        .map_err(crate::application_auth::AdminUpdateUsersError::from)
}
