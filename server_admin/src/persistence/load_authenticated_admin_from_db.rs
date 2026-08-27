pub(in super::super) async fn load_authenticated_admin_from_db(
    db: &mut super::AdminDbRef<'_, '_>,
    user_id: super::super::super::AdminUserId,
    session_id: super::super::super::AdminSessionId,
) -> Result<super::super::AuthenticatedAdmin, super::super::AdminError> {
    let user_query =
        sqlx::query_as::<_, (String, String, bool)>(constants_str::SERVER_ADMIN_READ_AUTH_USER_SQL)
            .bind(user_id.get());
    let optional_user = match db {
        super::AdminDbRef::Connection(connection) => {
            user_query.fetch_optional(&mut ***connection).await
        }
        super::AdminDbRef::Pool(pool) => user_query.fetch_optional(&***pool).await,
    }
    .map_err(super::super::super::SqlxAdminError::from)
    .map_err(super::super::AdminError::postgresql)?;
    let (raw_login, raw_display_name, must_change_password) =
        optional_user.ok_or(super::super::AdminError::Authentication)?;
    let roles_query =
        sqlx::query_scalar::<_, String>(constants_str::SERVER_ADMIN_READ_AUTH_ROLES_SQL)
            .bind(user_id.get());
    let raw_roles = match db {
        super::AdminDbRef::Connection(connection) => {
            roles_query.fetch_all(&mut ***connection).await
        }
        super::AdminDbRef::Pool(pool) => roles_query.fetch_all(&***pool).await,
    }
    .map_err(super::super::super::SqlxAdminError::from)
    .map_err(super::super::AdminError::postgresql)?;
    let permissions_query =
        sqlx::query_scalar::<_, String>(constants_str::SERVER_ADMIN_READ_AUTH_PERMISSIONS_SQL)
            .bind(user_id.get());
    let raw_permissions = match db {
        super::AdminDbRef::Connection(connection) => {
            permissions_query.fetch_all(&mut ***connection).await
        }
        super::AdminDbRef::Pool(pool) => permissions_query.fetch_all(&***pool).await,
    }
    .map_err(super::super::super::SqlxAdminError::from)
    .map_err(super::super::AdminError::postgresql)?;
    let display_name = super::super::super::AdminDisplayName::try_from(raw_display_name)
        .map_err(|_error| super::super::AdminError::Authentication)?;
    let login = super::super::super::AdminLogin::try_from(raw_login)
        .map_err(|_error| super::super::AdminError::Authentication)?;
    let password_change_required =
        super::super::super::AdminPasswordChangeRequired::from(must_change_password);
    let permissions = raw_permissions
        .into_iter()
        .map(|permission| super::super::super::AdminPermission::try_from(permission.as_str()))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_error| super::super::AdminError::Authentication)?
        .try_into()
        .map_err(|_error| super::super::AdminError::Authentication)?;
    let roles = raw_roles
        .into_iter()
        .map(super::super::super::AdminRoleName::try_from)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_error| super::super::AdminError::Authentication)?
        .try_into()
        .map_err(|_error| super::super::AdminError::Authentication)?;
    Ok(super::super::AuthenticatedAdmin {
        display_name,
        id: user_id,
        login,
        password_change_required,
        permissions,
        roles,
        session_id,
    })
}
