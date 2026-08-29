pub(crate) async fn load_authenticated_admin_from_db(
    db: &mut crate::admin_db_ref::AdminDbRef<'_, '_>,
    user_id: server_admin_core::admin_user_id::AdminUserId,
    session_id: crate::admin_session_id::AdminSessionId,
) -> Result<crate::authenticated_admin::AuthenticatedAdmin, crate::admin_error::AdminError> {
    let user_query = sqlx::query_as::<_, (String, String, bool)>(
        constants_str::integration_fixtures::SERVER_ADMIN_READ_AUTH_USER_SQL,
    )
    .bind(user_id.get());
    let optional_user = match db {
        crate::admin_db_ref::AdminDbRef::Connection(connection) => {
            user_query.fetch_optional(&mut ***connection).await
        }
        crate::admin_db_ref::AdminDbRef::Pool(pool) => user_query.fetch_optional(&***pool).await,
    }
    .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
    .map_err(crate::admin_error::AdminError::postgresql)?;
    let (raw_login, raw_display_name, must_change_password) =
        optional_user.ok_or(crate::admin_error::AdminError::Authentication)?;
    let roles_query = sqlx::query_scalar::<_, String>(
        constants_str::integration_fixtures::SERVER_ADMIN_READ_AUTH_ROLES_SQL,
    )
    .bind(user_id.get());
    let raw_roles = match db {
        crate::admin_db_ref::AdminDbRef::Connection(connection) => {
            roles_query.fetch_all(&mut ***connection).await
        }
        crate::admin_db_ref::AdminDbRef::Pool(pool) => roles_query.fetch_all(&***pool).await,
    }
    .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
    .map_err(crate::admin_error::AdminError::postgresql)?;
    let permissions_query = sqlx::query_scalar::<_, String>(
        constants_str::integration_fixtures::SERVER_ADMIN_READ_AUTH_PERMISSIONS_SQL,
    )
    .bind(user_id.get());
    let raw_permissions = match db {
        crate::admin_db_ref::AdminDbRef::Connection(connection) => {
            permissions_query.fetch_all(&mut ***connection).await
        }
        crate::admin_db_ref::AdminDbRef::Pool(pool) => permissions_query.fetch_all(&***pool).await,
    }
    .map_err(crate::sqlx_admin_error::SqlxAdminError::from)
    .map_err(crate::admin_error::AdminError::postgresql)?;
    let display_name =
        server_admin_contract::admin_display_name::AdminDisplayName::try_from(raw_display_name)
            .map_err(|_error| crate::admin_error::AdminError::Authentication)?;
    let login = server_admin_contract::admin_login::AdminLogin::try_from(raw_login)
        .map_err(|_error| crate::admin_error::AdminError::Authentication)?;
    let password_change_required =
        crate::admin_password_change_required::AdminPasswordChangeRequired::from(
            must_change_password,
        );
    let permissions = raw_permissions
        .into_iter()
        .map(|permission| {
            server_admin_contract::admin_permission::AdminPermission::try_from(permission.as_str())
        })
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_error| crate::admin_error::AdminError::Authentication)?
        .try_into()
        .map_err(|_error| crate::admin_error::AdminError::Authentication)?;
    let roles = raw_roles
        .into_iter()
        .map(server_admin_contract::admin_role_name::AdminRoleName::try_from)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_error| crate::admin_error::AdminError::Authentication)?
        .try_into()
        .map_err(|_error| crate::admin_error::AdminError::Authentication)?;
    Ok(crate::authenticated_admin::AuthenticatedAdmin {
        display_name,
        id: user_id,
        login,
        password_change_required,
        permissions,
        roles,
        session_id,
    })
}
