pub async fn reset_admin_password(
    pool: app_state::sqlx_pg_pool_ref::SqlxPgPoolRef<'_>,
    login: server_admin_contract::admin_login::AdminLogin,
    password: server_admin_contract::admin_new_password::AdminNewPassword,
    password_hasher: &crate::admin_password_hasher::AdminPasswordHasher,
) -> Result<
    server_admin_core::admin_user_record_id::AdminUserRecordId,
    crate::admin_password_reset_error::AdminPasswordResetError,
> {
    let password_hash = password_hasher
        .hash(
            crate::runtime_admin_password::RuntimeAdminPassword::try_from(password.into_inner())
                .map_err(|password_error| {
                    let _error_text = format!("{password_error:?}");
                    crate::admin_password_reset_error::AdminPasswordResetError::InvalidPassword
                })?,
        )
        .await
        .map_err(crate::admin_password_reset_error::AdminPasswordResetError::PasswordHash)?;
    let contract_login =
        server_admin_contract::admin_login::AdminLogin::try_from(login.as_ref().to_owned())
            .map_err(|error| {
                let _error_text = format!("{error:?}");
                crate::admin_password_reset_error::AdminPasswordResetError::InvalidLogin
            })?;
    let mut tx = pool.as_ref().begin().await.map_err(|error| {
        crate::admin_password_reset_error::AdminPasswordResetError::Pg(
            crate::sqlx_admin_error::SqlxAdminError::from(error),
        )
    })?;
    let _lock_result =
        sqlx::query(constants_str::integration_fixtures::SERVER_ADMIN_LOCK_USERS_SQL)
            .execute(&mut *tx)
            .await
            .map_err(|error| {
                crate::admin_password_reset_error::AdminPasswordResetError::Pg(
                    crate::sqlx_admin_error::SqlxAdminError::from(error),
                )
            })?;
    let optional_user_id = sqlx::query_scalar::<_, i64>(
        constants_str::integration_fixtures::SERVER_ADMIN_USER_ID_BY_LOGIN_SQL,
    )
    .bind(login.as_ref())
    .fetch_optional(&mut *tx)
    .await
    .map_err(|error| {
        crate::admin_password_reset_error::AdminPasswordResetError::Pg(
            crate::sqlx_admin_error::SqlxAdminError::from(error),
        )
    })?;
    let user_id = server_admin_core::admin_user_record_id::AdminUserRecordId::try_from(
        optional_user_id
            .ok_or(crate::admin_password_reset_error::AdminPasswordResetError::UnknownLogin)?,
    )
    .map_err(|error| {
        crate::admin_password_reset_error::AdminPasswordResetError::Pg(
            crate::sqlx_admin_error::SqlxAdminError::from(error),
        )
    })?;
    crate::update_user_password::update_user_password(
        crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(
            &mut *tx,
        ),
        user_id,
        &password_hash,
        crate::admin_password_change_required::AdminPasswordChangeRequired::from(true),
    )
    .await
    .map_err(crate::admin_password_reset_error::AdminPasswordResetError::Pg)?
    .get()
    .then_some(())
    .ok_or(crate::admin_password_reset_error::AdminPasswordResetError::UnknownLogin)?;
    crate::revoke_user_sessions::revoke_user_sessions(
        crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(
            &mut *tx,
        ),
        user_id,
    )
    .await
    .map_err(crate::admin_password_reset_error::AdminPasswordResetError::Pg)?;
    let resource_id =
        server_admin_core::std_admin_string::StdAdminString::try_from(user_id.to_string())
            .map_err(|error| {
                let _error_text = format!("{error:?}");
                crate::admin_password_reset_error::AdminPasswordResetError::AuditDetails
            })?;
    let details = server_admin_contract::serde_json_admin_audit_details::SerdeJsonAdminAuditDetails::try_from(
        serde_json::json!({ "operation": "password_reset", "target_id": resource_id.as_ref() }),
    )
    .map_err(|error| {
        let _error_text = format!("{error:?}");
        crate::admin_password_reset_error::AdminPasswordResetError::AuditDetails
    })?;
    crate::insert_audit_success::insert_audit_success(
        crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(
            &mut *tx,
        ),
        user_id,
        &contract_login,
        crate::admin_audit_action::AdminAuditAction::Update,
        crate::admin_audit_resource::AdminAuditResource::User,
        &resource_id,
        server_admin_core::uuid_admin_value::UuidAdminValue::from(uuid::Uuid::new_v4()),
        &details,
    )
    .await
    .map_err(crate::admin_password_reset_error::AdminPasswordResetError::Pg)?;
    tx.commit().await.map_err(|error| {
        crate::admin_password_reset_error::AdminPasswordResetError::Pg(
            crate::sqlx_admin_error::SqlxAdminError::from(error),
        )
    })?;
    Ok(user_id)
}
