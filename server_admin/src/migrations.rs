#![allow(clippy::single_call_fn)] // stable root migration and initial-administrator-creation API delegates to the private persistence module
static ADMIN_MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../server_admin_migrations");
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::DerefInner,
    newtype::FromInner,
)]
pub(crate) struct SqlxAdminMigratorRef(&'static sqlx::migrate::Migrator);
pub(crate) fn migrator() -> SqlxAdminMigratorRef {
    SqlxAdminMigratorRef::from(&ADMIN_MIGRATOR)
}
pub(crate) async fn create_initial_administrator(
    pool: app_state::domain_types::SqlxPgPoolRef<'_>,
    login: crate::domain_types::AdminLogin,
    display_name: crate::domain_types::AdminDisplayName,
    password: server_admin_contract::domain_types::AdminNewPassword,
    password_hasher: &crate::domain_types::AdminPasswordHasher,
) -> Result<crate::domain_types::AdminUserId, crate::domain_types::InitialAdministratorCreationError>
{
    let password_hash = password_hasher
        .hash(
            crate::domain_types::AdminPassword::try_from(password.into_inner()).map_err(
                |password_error| {
                    let _error_text = format!("{password_error:?}");
                    crate::domain_types::InitialAdministratorCreationError::InvalidPassword
                },
            )?,
        )
        .await
        .map_err(crate::domain_types::InitialAdministratorCreationError::PasswordHash)?;
    let mut tx = pool.as_ref().begin().await.map_err(|error| {
        crate::domain_types::InitialAdministratorCreationError::Pg(
            crate::domain_types::SqlxAdminError::from(error),
        )
    })?;
    let _lock_result = sqlx::query(constants_str::SERVER_ADMIN_LOCK_USERS_SQL)
        .execute(&mut *tx)
        .await
        .map_err(|error| {
            crate::domain_types::InitialAdministratorCreationError::Pg(
                crate::domain_types::SqlxAdminError::from(error),
            )
        })?;
    let user_exists = sqlx::query_scalar::<_, bool>(constants_str::SERVER_ADMIN_USERS_EXIST_SQL)
        .fetch_one(&mut *tx)
        .await
        .map_err(|error| {
            crate::domain_types::InitialAdministratorCreationError::Pg(
                crate::domain_types::SqlxAdminError::from(error),
            )
        })?;
    if user_exists {
        return Err(crate::domain_types::InitialAdministratorCreationError::AlreadyInitialized);
    }
    let user_id = crate::adapters::repository::insert_user::insert_user(
        crate::adapters::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        &login,
        &display_name,
        &password_hash,
    )
    .await
    .map_err(crate::domain_types::InitialAdministratorCreationError::Pg)?;
    let _role_link_result = sqlx::query(constants_str::SERVER_ADMIN_INSERT_ADMIN_ROLE_SQL)
        .bind(user_id.get())
        .execute(&mut *tx)
        .await
        .map_err(|error| {
            crate::domain_types::InitialAdministratorCreationError::Pg(
                crate::domain_types::SqlxAdminError::from(error),
            )
        })?;
    let contract_login =
        server_admin_contract::domain_types::AdminLogin::try_from(login.as_ref().to_owned())
            .map_err(|error| {
                let _error_text = format!("{error:?}");
                crate::domain_types::InitialAdministratorCreationError::InvalidLogin
            })?;
    let resource_id =
        crate::domain_types::StdAdminString::try_from(user_id.to_string()).map_err(|error| {
            let _error_text = format!("{error:?}");
            crate::domain_types::InitialAdministratorCreationError::AuditDetails
        })?;
    let details = server_admin_contract::domain_types::SerdeJsonAdminAuditDetails::try_from(
        serde_json::json!({ "operation": "initial_administrator_creation", "target_id": resource_id.as_ref() }),
    )
    .map_err(|error| {
        let _error_text = format!("{error:?}");
        crate::domain_types::InitialAdministratorCreationError::AuditDetails
    })?;
    crate::adapters::repository::insert_audit_success::insert_audit_success(
        crate::adapters::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        user_id,
        &contract_login,
        crate::domain_types::AdminAuditAction::Create,
        crate::domain_types::AdminAuditResource::User,
        &resource_id,
        crate::domain_types::UuidAdminValue::from(uuid::Uuid::new_v4()),
        &details,
    )
    .await
    .map_err(crate::domain_types::InitialAdministratorCreationError::Pg)?;
    tx.commit().await.map_err(|error| {
        crate::domain_types::InitialAdministratorCreationError::Pg(
            crate::domain_types::SqlxAdminError::from(error),
        )
    })?;
    Ok(user_id)
}

pub(crate) async fn reset_admin_password(
    pool: app_state::domain_types::SqlxPgPoolRef<'_>,
    login: crate::domain_types::AdminLogin,
    password: server_admin_contract::domain_types::AdminNewPassword,
    password_hasher: &crate::domain_types::AdminPasswordHasher,
) -> Result<crate::domain_types::AdminUserId, crate::domain_types::AdminPasswordResetError> {
    let password_hash = password_hasher
        .hash(
            crate::domain_types::AdminPassword::try_from(password.into_inner()).map_err(
                |password_error| {
                    let _error_text = format!("{password_error:?}");
                    crate::domain_types::AdminPasswordResetError::InvalidPassword
                },
            )?,
        )
        .await
        .map_err(crate::domain_types::AdminPasswordResetError::PasswordHash)?;
    let contract_login =
        server_admin_contract::domain_types::AdminLogin::try_from(login.as_ref().to_owned())
            .map_err(|error| {
                let _error_text = format!("{error:?}");
                crate::domain_types::AdminPasswordResetError::InvalidLogin
            })?;
    let mut tx = pool.as_ref().begin().await.map_err(|error| {
        crate::domain_types::AdminPasswordResetError::Pg(crate::domain_types::SqlxAdminError::from(
            error,
        ))
    })?;
    let _lock_result = sqlx::query(constants_str::SERVER_ADMIN_LOCK_USERS_SQL)
        .execute(&mut *tx)
        .await
        .map_err(|error| {
            crate::domain_types::AdminPasswordResetError::Pg(
                crate::domain_types::SqlxAdminError::from(error),
            )
        })?;
    let optional_user_id =
        sqlx::query_scalar::<_, i64>(constants_str::SERVER_ADMIN_USER_ID_BY_LOGIN_SQL)
            .bind(login.as_ref())
            .fetch_optional(&mut *tx)
            .await
            .map_err(|error| {
                crate::domain_types::AdminPasswordResetError::Pg(
                    crate::domain_types::SqlxAdminError::from(error),
                )
            })?;
    let user_id = crate::domain_types::AdminUserId::try_from(
        optional_user_id.ok_or(crate::domain_types::AdminPasswordResetError::UnknownLogin)?,
    )
    .map_err(|error| {
        crate::domain_types::AdminPasswordResetError::Pg(crate::domain_types::SqlxAdminError::from(
            error,
        ))
    })?;
    crate::adapters::repository::update_user_password::update_user_password(
        crate::adapters::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        user_id,
        &password_hash,
        crate::domain_types::AdminPasswordChangeRequired::from(true),
    )
    .await
    .map_err(crate::domain_types::AdminPasswordResetError::Pg)?
    .get()
    .then_some(())
    .ok_or(crate::domain_types::AdminPasswordResetError::UnknownLogin)?;
    crate::adapters::repository::revoke_user_sessions::revoke_user_sessions(
        crate::adapters::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        user_id,
    )
    .await
    .map_err(crate::domain_types::AdminPasswordResetError::Pg)?;
    let resource_id =
        crate::domain_types::StdAdminString::try_from(user_id.to_string()).map_err(|error| {
            let _error_text = format!("{error:?}");
            crate::domain_types::AdminPasswordResetError::AuditDetails
        })?;
    let details = server_admin_contract::domain_types::SerdeJsonAdminAuditDetails::try_from(
        serde_json::json!({ "operation": "password_reset", "target_id": resource_id.as_ref() }),
    )
    .map_err(|error| {
        let _error_text = format!("{error:?}");
        crate::domain_types::AdminPasswordResetError::AuditDetails
    })?;
    crate::adapters::repository::insert_audit_success::insert_audit_success(
        crate::adapters::repository::SqlxAdminRepositoryConnectionMutRef::from(&mut *tx),
        user_id,
        &contract_login,
        crate::domain_types::AdminAuditAction::Update,
        crate::domain_types::AdminAuditResource::User,
        &resource_id,
        crate::domain_types::UuidAdminValue::from(uuid::Uuid::new_v4()),
        &details,
    )
    .await
    .map_err(crate::domain_types::AdminPasswordResetError::Pg)?;
    tx.commit().await.map_err(|error| {
        crate::domain_types::AdminPasswordResetError::Pg(crate::domain_types::SqlxAdminError::from(
            error,
        ))
    })?;
    Ok(user_id)
}
