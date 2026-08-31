pub(crate) async fn account_change_own_password(
    auth: crate::admin_auth_req::AdminAuthReq,
    request: crate::axum_admin_json::AxumAdminJson<
        server_admin_contract::admin_change_own_password_req::AdminChangeOwnPasswordReq,
    >,
) -> Result<crate::axum_admin_response::AxumAdminResponse, crate::admin_error::AdminError> {
    let actor = crate::authorization_authenticate::authorization_authenticate(
        auth.get_state().as_ref(),
        crate::http_admin_header_map_ref::HttpAdminHeaderMapRef::from(auth.get_headers().as_ref()),
        *auth.get_peer(),
    )
    .await?;
    let subject = server_admin_core::std_admin_string::StdAdminString::try_from(
        actor.get_id().get().to_string(),
    )
    .map_err(|_error| crate::admin_error::AdminError::Validation)?;
    crate::enforce_rate_limit::enforce_rate_limit(
        auth.get_state().as_ref(),
        crate::admin_rate_limit_scope::AdminRateLimitScope::Mutation,
        &subject,
        auth.get_state().as_ref().get_policy().get_mutation_limit(),
        auth.get_state().as_ref().get_policy().get_mutation_window(),
    )
    .await?;
    crate::authorization_validate_csrf::authorization_validate_csrf(
        auth.get_state().as_ref(),
        crate::http_admin_header_map_ref::HttpAdminHeaderMapRef::from(auth.get_headers().as_ref()),
        &actor,
    )
    .await?;
    let (current_password, new_password) = request.into_inner().into_parts();
    let expected_hash =
        sqlx::query_scalar::<_, String>(constants_str::SERVER_ADMIN_READ_PASSWORD_HASH_SQL)
            .bind(actor.get_id().get())
            .fetch_optional(auth.get_state().as_ref().get_pool().as_ref())
            .await
            .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
            .map(|value| {
                value.map(|hash| {
                    crate::admin_password_hash::AdminPasswordHash::new(
                        pg_types_text_misc::generate_pg_types_mod::StringAsNonNullTextSecret::from(
                            hash,
                        ),
                    )
                })
            })
            .map_err(crate::admin_error::AdminError::from)?
            .ok_or(crate::admin_error::AdminError::Authentication)?;
    if !auth
        .get_state()
        .as_ref()
        .get_password_hasher()
        .verify(
            crate::admin_password_from_contract::admin_password_from_contract(current_password)
                .map_err(crate::admin_error::AdminError::password_text)?,
            expected_hash,
        )
        .await
        .map_err(crate::admin_error::AdminError::password_hash)?
        .get()
    {
        return Err(crate::admin_error::AdminError::Validation);
    }
    let password_hash = auth
        .get_state()
        .as_ref()
        .get_password_hasher()
        .hash(
            crate::admin_new_password_from_contract::admin_new_password_from_contract(new_password)
                .map_err(crate::admin_error::AdminError::password_text)?,
        )
        .await
        .map_err(crate::admin_error::AdminError::password_hash)?;
    let mut tx = auth
        .get_state()
        .as_ref()
        .get_pool()
        .as_ref()
        .begin()
        .await
        .map_err(crate::admin_error::AdminError::from)?;
    crate::update_user_password::update_user_password(
        crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(
            &mut *tx,
        ),
        *actor.get_id(),
        &password_hash,
        crate::admin_password_change_required::AdminPasswordChangeRequired::from(false),
    )
    .await
    .map_err(crate::admin_error::AdminError::from)?
    .get()
    .then_some(())
    .ok_or(crate::admin_error::AdminError::Conflict)?;
    sqlx::query(constants_str::SERVER_ADMIN_REVOKE_OTHER_ACCESS_SESSIONS_SQL)
        .bind(actor.get_id().get())
        .bind(actor.get_session_id().get().get())
        .execute(&mut *tx)
        .await
        .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
        .map_err(crate::admin_error::AdminError::from)
        .map(drop)?;
    sqlx::query(constants_str::SERVER_ADMIN_REVOKE_USER_REFRESH_TOKENS_SQL)
        .bind(actor.get_id().get())
        .execute(&mut *tx)
        .await
        .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
        .map_err(crate::admin_error::AdminError::from)
        .map(drop)?;
    crate::record_audit_success_in_connection::record_audit_success_in_connection(
        crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(
            &mut *tx,
        ),
        crate::admin_audit_success_ref::AdminAuditSuccessRef::new(
            crate::admin_audit_action::AdminAuditAction::Update,
            actor.get_login(),
            crate::admin_audit_resource::AdminAuditResource::User,
            crate::admin_audit_resource_id::AdminAuditResourceId::User(*actor.get_id()),
            *actor.get_id(),
        ),
    )
    .await?;
    tx.commit()
        .await
        .map_err(crate::admin_error::AdminError::from)?;
    Ok(crate::axum_admin_response::AxumAdminResponse::from(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
