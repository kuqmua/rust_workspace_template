pub async fn create_initial_administrator(
    sqlx_pg_pool_ref: app_state::sqlx_pg_pool_ref::SqlxPgPoolRef<'_>,
    admin_login: server_admin_contract::admin_login::AdminLogin,
    admin_display_name: server_admin_contract::admin_display_name::AdminDisplayName,
    admin_new_password: server_admin_contract::admin_new_password::AdminNewPassword,
    admin_password_hasher: &crate::admin_password_hasher::AdminPasswordHasher,
) -> Result<
    server_admin_core::admin_user_record_id::AdminUserRecordId,
    crate::initial_administrator_creation_error::InitialAdministratorCreationError,
> {
    let password_hash = admin_password_hasher
        .hash(
            crate::runtime_admin_password::RuntimeAdminPassword::try_from(admin_new_password.into_inner()).map_err(
                |password_error| {
                    let _error_text = format!("{password_error:?}");
                    crate::initial_administrator_creation_error::InitialAdministratorCreationError::InvalidPassword
                },
            )?,
        )
        .await
        .map_err(crate::initial_administrator_creation_error::InitialAdministratorCreationError::PasswordHash)?;
    let mut tx = sqlx_pg_pool_ref.as_ref().begin().await.map_err(|error| {
        crate::initial_administrator_creation_error::InitialAdministratorCreationError::Pg(
            crate::sqlx_admin_error::SqlxAdminError::from(error),
        )
    })?;
    let _lock_result = sqlx::query(constants_str::SERVER_ADMIN_LOCK_USERS_SQL)
        .execute(&mut *tx)
        .await
        .map_err(|error| {
            crate::initial_administrator_creation_error::InitialAdministratorCreationError::Pg(
                crate::sqlx_admin_error::SqlxAdminError::from(error),
            )
        })?;
    let user_exists = sqlx::query_scalar::<_, bool>(constants_str::SERVER_ADMIN_USERS_EXIST_SQL)
        .fetch_one(&mut *tx)
        .await
        .map_err(|error| {
            crate::initial_administrator_creation_error::InitialAdministratorCreationError::Pg(
                crate::sqlx_admin_error::SqlxAdminError::from(error),
            )
        })?;
    if user_exists {
        return Err(crate::initial_administrator_creation_error::InitialAdministratorCreationError::AlreadyInitialized);
    }
    let user_id = crate::insert_user::insert_user(
        crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(
            &mut *tx,
        ),
        &admin_login,
        &admin_display_name,
        &password_hash,
    )
    .await
    .map_err(crate::initial_administrator_creation_error::InitialAdministratorCreationError::Pg)?;
    let _role_link_result = sqlx::query(constants_str::SERVER_ADMIN_INSERT_ADMIN_ROLE_SQL)
        .bind(user_id.get())
        .execute(&mut *tx)
        .await
        .map_err(|error| {
            crate::initial_administrator_creation_error::InitialAdministratorCreationError::Pg(
                crate::sqlx_admin_error::SqlxAdminError::from(error),
            )
        })?;
    let contract_login = server_admin_contract::admin_login::AdminLogin::try_from(
        admin_login.as_ref().to_owned(),
    )
    .map_err(|error| {
        let _error_text = format!("{error:?}");
        crate::initial_administrator_creation_error::InitialAdministratorCreationError::InvalidLogin
    })?;
    let resource_id = server_admin_core::std_admin_string::StdAdminString::try_from(
        user_id.to_string(),
    )
    .map_err(|error| {
        let _error_text = format!("{error:?}");
        crate::initial_administrator_creation_error::InitialAdministratorCreationError::AuditDetails
    })?;
    let details = server_admin_contract::serde_json_admin_audit_details::SerdeJsonAdminAuditDetails::try_from(
        serde_json::json!({ "operation": "initial_administrator_creation", "target_id": resource_id.as_ref() }),
    )
    .map_err(|error| {
        let _error_text = format!("{error:?}");
        crate::initial_administrator_creation_error::InitialAdministratorCreationError::AuditDetails
    })?;
    crate::insert_audit_success::insert_audit_success(
        crate::sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef::from(
            &mut *tx,
        ),
        user_id,
        &contract_login,
        crate::admin_audit_action::AdminAuditAction::Create,
        crate::admin_audit_resource::AdminAuditResource::User,
        &resource_id,
        server_admin_core::uuid_admin_value::UuidAdminValue::from(uuid::Uuid::new_v4()),
        &details,
    )
    .await
    .map_err(crate::initial_administrator_creation_error::InitialAdministratorCreationError::Pg)?;
    tx.commit().await.map_err(|error| {
        crate::initial_administrator_creation_error::InitialAdministratorCreationError::Pg(
            crate::sqlx_admin_error::SqlxAdminError::from(error),
        )
    })?;
    Ok(user_id)
}
