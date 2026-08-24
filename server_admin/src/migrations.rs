#![allow(clippy::single_call_fn)] // stable root migration/bootstrap API delegates to the private persistence module
static ADMIN_MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");
#[cfg(test)]
pub(super) const fn migrator() -> &'static sqlx::migrate::Migrator {
    &ADMIN_MIGRATOR
}
pub(super) async fn prep_pg(
    pool: app_state::domain_types::SqlxPgPoolRef<'_>,
) -> Result<(), super::AdminMigrateError> {
    ADMIN_MIGRATOR.run(pool.as_ref()).await.map_err(|error| {
        super::AdminMigrateError(super::AdminMigrateErrorInner::Migration(
            super::SqlxAdminMigrateError::from(error),
        ))
    })?;
    let permission_names = server_admin_contract::AdminPermission::ALL
        .into_iter()
        .map(|permission| permission.as_str().as_ref().to_owned())
        .collect::<Vec<_>>();
    let _permission_result = sqlx::query(constants_str::SERVER_ADMIN_RECONCILE_PERMISSIONS_SQL)
        .bind(permission_names)
        .execute(pool.as_ref())
        .await
        .map_err(|error| {
            super::AdminMigrateError(super::AdminMigrateErrorInner::Reconciliation(
                super::SqlxAdminError::from(error),
            ))
        })?;
    let _role_permission_result =
        sqlx::query(constants_str::SERVER_ADMIN_RECONCILE_ROLE_PERMISSIONS_SQL)
            .execute(pool.as_ref())
            .await
            .map_err(|error| {
                super::AdminMigrateError(super::AdminMigrateErrorInner::Reconciliation(
                    super::SqlxAdminError::from(error),
                ))
            })?;
    Ok(())
}
pub(super) async fn bootstrap_admin(
    pool: app_state::domain_types::SqlxPgPoolRef<'_>,
    login: super::AdminLogin,
    display_name: super::AdminDisplayName,
    password: server_admin_contract::AdminNewPassword,
    password_hasher: &super::AdminPasswordHasher,
) -> Result<super::AdminUserId, super::AdminBootstrapError> {
    let password_hash = password_hasher
        .hash(
            super::AdminPassword::try_from(password.into_inner()).map_err(|password_error| {
                let _error_text = format!("{password_error:?}");
                super::AdminBootstrapError::InvalidPassword
            })?,
        )
        .await
        .map_err(super::AdminBootstrapError::PasswordHash)?;
    let mut tx = pool
        .as_ref()
        .begin()
        .await
        .map_err(|error| super::AdminBootstrapError::Pg(super::SqlxAdminError::from(error)))?;
    let _lock_result = sqlx::query(constants_str::SERVER_ADMIN_LOCK_USERS_SQL)
        .execute(&mut *tx)
        .await
        .map_err(|error| super::AdminBootstrapError::Pg(super::SqlxAdminError::from(error)))?;
    let user_exists = sqlx::query_scalar::<_, bool>(constants_str::SERVER_ADMIN_USERS_EXIST_SQL)
        .fetch_one(&mut *tx)
        .await
        .map_err(|error| super::AdminBootstrapError::Pg(super::SqlxAdminError::from(error)))?;
    if user_exists {
        return Err(super::AdminBootstrapError::AlreadyInitialized);
    }
    let user_id = super::repository::users::insert_user(
        super::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        &login,
        &display_name,
        &password_hash,
    )
    .await
    .map_err(super::AdminBootstrapError::Pg)?;
    let _role_link_result = sqlx::query(constants_str::SERVER_ADMIN_INSERT_ADMIN_ROLE_SQL)
        .bind(user_id.get())
        .execute(&mut *tx)
        .await
        .map_err(|error| super::AdminBootstrapError::Pg(super::SqlxAdminError::from(error)))?;
    let contract_login = server_admin_contract::AdminLogin::try_from(login.as_ref().to_owned())
        .map_err(|error| {
            let _error_text = format!("{error:?}");
            super::AdminBootstrapError::InvalidLogin
        })?;
    let resource_id = super::StdAdminString::try_from(user_id.to_string()).map_err(|error| {
        let _error_text = format!("{error:?}");
        super::AdminBootstrapError::AuditDetails
    })?;
    let details = server_admin_contract::SerdeJsonAdminAuditDetails::try_from(
        serde_json::json!({ "operation": "bootstrap", "target_id": resource_id.as_ref() }),
    )
    .map_err(|error| {
        let _error_text = format!("{error:?}");
        super::AdminBootstrapError::AuditDetails
    })?;
    super::repository::audit::insert_audit_success(
        super::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        user_id,
        &contract_login,
        super::AdminAuditAction::Create,
        super::AdminAuditResource::User,
        &resource_id,
        super::UuidAdminValue::from(uuid::Uuid::new_v4()),
        &details,
    )
    .await
    .map_err(super::AdminBootstrapError::Pg)?;
    tx.commit()
        .await
        .map_err(|error| super::AdminBootstrapError::Pg(super::SqlxAdminError::from(error)))?;
    Ok(user_id)
}

pub(super) async fn reset_admin_password(
    pool: app_state::domain_types::SqlxPgPoolRef<'_>,
    login: super::AdminLogin,
    password: server_admin_contract::AdminNewPassword,
    password_hasher: &super::AdminPasswordHasher,
) -> Result<super::AdminUserId, super::AdminPasswordResetError> {
    let password_hash = password_hasher
        .hash(
            super::AdminPassword::try_from(password.into_inner()).map_err(|password_error| {
                let _error_text = format!("{password_error:?}");
                super::AdminPasswordResetError::InvalidPassword
            })?,
        )
        .await
        .map_err(super::AdminPasswordResetError::PasswordHash)?;
    let contract_login = server_admin_contract::AdminLogin::try_from(login.as_ref().to_owned())
        .map_err(|error| {
            let _error_text = format!("{error:?}");
            super::AdminPasswordResetError::InvalidLogin
        })?;
    let mut tx =
        pool.as_ref().begin().await.map_err(|error| {
            super::AdminPasswordResetError::Pg(super::SqlxAdminError::from(error))
        })?;
    let _lock_result = sqlx::query(constants_str::SERVER_ADMIN_LOCK_USERS_SQL)
        .execute(&mut *tx)
        .await
        .map_err(|error| super::AdminPasswordResetError::Pg(super::SqlxAdminError::from(error)))?;
    let optional_user_id =
        sqlx::query_scalar::<_, i64>(constants_str::SERVER_ADMIN_USER_ID_BY_LOGIN_SQL)
            .bind(login.as_ref())
            .fetch_optional(&mut *tx)
            .await
            .map_err(|error| {
                super::AdminPasswordResetError::Pg(super::SqlxAdminError::from(error))
            })?;
    let user_id = super::AdminUserId::try_from(
        optional_user_id.ok_or(super::AdminPasswordResetError::UnknownLogin)?,
    )
    .map_err(|error| super::AdminPasswordResetError::Pg(super::SqlxAdminError::from(error)))?;
    super::repository::users::update_user_password(
        super::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        user_id,
        &password_hash,
        super::AdminPasswordChangeRequired::from(true),
    )
    .await
    .map_err(super::AdminPasswordResetError::Pg)?
    .get()
    .then_some(())
    .ok_or(super::AdminPasswordResetError::UnknownLogin)?;
    super::repository::sessions::revoke_user_sessions(
        super::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        user_id,
    )
    .await
    .map_err(super::AdminPasswordResetError::Pg)?;
    let resource_id = super::StdAdminString::try_from(user_id.to_string()).map_err(|error| {
        let _error_text = format!("{error:?}");
        super::AdminPasswordResetError::AuditDetails
    })?;
    let details = server_admin_contract::SerdeJsonAdminAuditDetails::try_from(
        serde_json::json!({ "operation": "password_reset", "target_id": resource_id.as_ref() }),
    )
    .map_err(|error| {
        let _error_text = format!("{error:?}");
        super::AdminPasswordResetError::AuditDetails
    })?;
    super::repository::audit::insert_audit_success(
        super::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        user_id,
        &contract_login,
        super::AdminAuditAction::Update,
        super::AdminAuditResource::User,
        &resource_id,
        super::UuidAdminValue::from(uuid::Uuid::new_v4()),
        &details,
    )
    .await
    .map_err(super::AdminPasswordResetError::Pg)?;
    tx.commit()
        .await
        .map_err(|error| super::AdminPasswordResetError::Pg(super::SqlxAdminError::from(error)))?;
    Ok(user_id)
}
