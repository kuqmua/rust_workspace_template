pub(crate) async fn load_authenticated_admin_from_db(
    db: &mut crate::AdminDbRef<'_, '_>,
    user_id: crate::AdminUserId,
    session_id: crate::AdminSessionId,
) -> Result<crate::AuthenticatedAdmin, crate::AdminError> {
    let user_query =
        sqlx::query_as::<_, (String, String, bool)>(constants_str::SERVER_ADMIN_READ_AUTH_USER_SQL)
            .bind(user_id.get());
    let optional_user = match db {
        crate::AdminDbRef::Connection(connection) => {
            user_query.fetch_optional(&mut ***connection).await
        }
        crate::AdminDbRef::Pool(pool) => user_query.fetch_optional(&***pool).await,
    }
    .map_err(crate::SqlxAdminError::from)
    .map_err(crate::AdminError::postgresql)?;
    let (raw_login, raw_display_name, must_change_password) =
        optional_user.ok_or(crate::AdminError::Authentication)?;
    let roles_query =
        sqlx::query_scalar::<_, String>(constants_str::SERVER_ADMIN_READ_AUTH_ROLES_SQL)
            .bind(user_id.get());
    let raw_roles = match db {
        crate::AdminDbRef::Connection(connection) => {
            roles_query.fetch_all(&mut ***connection).await
        }
        crate::AdminDbRef::Pool(pool) => roles_query.fetch_all(&***pool).await,
    }
    .map_err(crate::SqlxAdminError::from)
    .map_err(crate::AdminError::postgresql)?;
    let permissions_query =
        sqlx::query_scalar::<_, String>(constants_str::SERVER_ADMIN_READ_AUTH_PERMISSIONS_SQL)
            .bind(user_id.get());
    let raw_permissions = match db {
        crate::AdminDbRef::Connection(connection) => {
            permissions_query.fetch_all(&mut ***connection).await
        }
        crate::AdminDbRef::Pool(pool) => permissions_query.fetch_all(&***pool).await,
    }
    .map_err(crate::SqlxAdminError::from)
    .map_err(crate::AdminError::postgresql)?;
    let display_name = crate::AdminDisplayName::try_from(raw_display_name)
        .map_err(|_error| crate::AdminError::Authentication)?;
    let login = crate::AdminLogin::try_from(raw_login)
        .map_err(|_error| crate::AdminError::Authentication)?;
    let password_change_required = crate::AdminPasswordChangeRequired::from(must_change_password);
    let permissions = raw_permissions
        .into_iter()
        .map(|permission| crate::AdminPermission::try_from(permission.as_str()))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_error| crate::AdminError::Authentication)?
        .try_into()
        .map_err(|_error| crate::AdminError::Authentication)?;
    let roles = raw_roles
        .into_iter()
        .map(crate::AdminRoleName::try_from)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_error| crate::AdminError::Authentication)?
        .try_into()
        .map_err(|_error| crate::AdminError::Authentication)?;
    Ok(crate::AuthenticatedAdmin {
        display_name,
        id: user_id,
        login,
        password_change_required,
        permissions,
        roles,
        session_id,
    })
}
