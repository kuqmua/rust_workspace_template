#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the application-layer audit event is constructed by sibling workflows and persisted only by this module"
)]

pub(super) async fn record_login_attempt(
    state: &super::AdminAuthSvcState,
    login: &super::super::AdminLogin,
    peer: super::AdminPeerAddr,
    succeeded: super::super::StdAdminBool,
) -> Result<(), super::AdminError> {
    sqlx::query(constants_str::SERVER_ADMIN_RECORD_LOGIN_ATTEMPT_SQL)
        .bind(login.as_ref())
        .bind(peer.socket_addr().get().ip())
        .bind(succeeded.get())
        .bind(uuid::Uuid::new_v4())
        .execute(state.pool.as_ref())
        .await
        .map_err(super::super::SqlxAdminError::from)
        .map(drop)
        .map_err(super::AdminError::postgresql)
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy)]
pub(super) struct AdminAuditSuccessRef<'value_lt> {
    pub(super) action: super::super::AdminAuditAction,
    pub(super) login: &'value_lt super::super::AdminLogin,
    pub(super) resource: super::super::AdminAuditResource,
    pub(super) resource_id: AdminAuditResourceId,
    pub(super) user_id: super::super::AdminUserId,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy)]
pub(super) enum AdminAuditResourceId {
    Role(super::super::AdminRoleId),
    Session(super::super::AdminSessionId),
    SystemSettings,
    User(super::super::AdminUserId),
}
impl AdminAuditResourceId {
    pub(super) fn value(self) -> super::super::StdAdminString {
        match self {
            Self::User(value) => super::super::StdAdminString::from_positive_i64(value.value()),
            Self::Role(value) => super::super::StdAdminString::from_positive_i64(value.value()),
            Self::Session(value) => super::super::StdAdminString::from_uuid(value.get()),
            Self::SystemSettings => super::super::StdAdminString::system_settings_resource(),
        }
    }
}
pub(super) async fn record_audit_success_in_connection(
    mut connection: SqlxAdminPgConnectionRef<'_>,
    event: AdminAuditSuccessRef<'_>,
) -> Result<(), super::AdminError> {
    let details = server_admin_contract::domain_types::SerdeJsonAdminAuditDetails::try_from(
        serde_json::json!({ "operation": event.action.as_str().as_ref(), "target_id": event.resource_id.value().as_ref() }),
    )
    .map_err(|_error| super::AdminError::Validation)?;
    let resource_id = event.resource_id.value();
    crate::adapters::repository::insert_audit_success::insert_audit_success(
        crate::adapters::repository::SqlxAdminRepositoryConnectionMutRef::from(connection.as_mut()),
        event.user_id,
        event.login,
        event.action,
        event.resource,
        &resource_id,
        super::super::UuidAdminValue::from(uuid::Uuid::new_v4()),
        &details,
    )
    .await
    .map_err(super::AdminError::postgresql)
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::AsMut, newtype::FromInner)]
pub(super) struct SqlxAdminPgConnectionRef<'connection_lt>(&'connection_lt mut sqlx::PgConnection);

pub(super) async fn load_authenticated_admin(
    state: &super::AdminAuthSvcState,
    user_id: super::super::AdminUserId,
    session_id: super::super::AdminSessionId,
) -> Result<super::AuthenticatedAdmin, super::AdminError> {
    let mut db = AdminDbRef::Pool(
        crate::adapters::repository::SqlxAdminRepositoryPoolRef::from(state.pool.as_ref()),
    );
    load_authenticated_admin_from_db(&mut db, user_id, session_id).await
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) enum AdminDbRef<'connection_lt, 'pool_lt> {
    Connection(crate::adapters::repository::SqlxAdminRepositoryConnectionMutRef<'connection_lt>),
    Pool(crate::adapters::repository::SqlxAdminRepositoryPoolRef<'pool_lt>),
}
pub(super) async fn load_authenticated_admin_from_db(
    db: &mut AdminDbRef<'_, '_>,
    user_id: super::super::AdminUserId,
    session_id: super::super::AdminSessionId,
) -> Result<super::AuthenticatedAdmin, super::AdminError> {
    let user_query =
        sqlx::query_as::<_, (String, String, bool)>(constants_str::SERVER_ADMIN_READ_AUTH_USER_SQL)
            .bind(user_id.get());
    let optional_user = match db {
        AdminDbRef::Connection(connection) => user_query.fetch_optional(&mut ***connection).await,
        AdminDbRef::Pool(pool) => user_query.fetch_optional(&***pool).await,
    }
    .map_err(super::super::SqlxAdminError::from)
    .map_err(super::AdminError::postgresql)?;
    let (raw_login, raw_display_name, must_change_password) =
        optional_user.ok_or(super::AdminError::Authentication)?;
    let roles_query =
        sqlx::query_scalar::<_, String>(constants_str::SERVER_ADMIN_READ_AUTH_ROLES_SQL)
            .bind(user_id.get());
    let raw_roles = match db {
        AdminDbRef::Connection(connection) => roles_query.fetch_all(&mut ***connection).await,
        AdminDbRef::Pool(pool) => roles_query.fetch_all(&***pool).await,
    }
    .map_err(super::super::SqlxAdminError::from)
    .map_err(super::AdminError::postgresql)?;
    let permissions_query =
        sqlx::query_scalar::<_, String>(constants_str::SERVER_ADMIN_READ_AUTH_PERMISSIONS_SQL)
            .bind(user_id.get());
    let raw_permissions = match db {
        AdminDbRef::Connection(connection) => permissions_query.fetch_all(&mut ***connection).await,
        AdminDbRef::Pool(pool) => permissions_query.fetch_all(&***pool).await,
    }
    .map_err(super::super::SqlxAdminError::from)
    .map_err(super::AdminError::postgresql)?;
    let display_name = super::super::AdminDisplayName::try_from(raw_display_name)
        .map_err(|_error| super::AdminError::Authentication)?;
    let login = super::super::AdminLogin::try_from(raw_login)
        .map_err(|_error| super::AdminError::Authentication)?;
    let password_change_required =
        super::super::AdminPasswordChangeRequired::from(must_change_password);
    let permissions = raw_permissions
        .into_iter()
        .map(|permission| super::super::AdminPermission::try_from(permission.as_str()))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_error| super::AdminError::Authentication)?
        .try_into()
        .map_err(|_error| super::AdminError::Authentication)?;
    let roles = raw_roles
        .into_iter()
        .map(super::super::AdminRoleName::try_from)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_error| super::AdminError::Authentication)?
        .try_into()
        .map_err(|_error| super::AdminError::Authentication)?;
    Ok(super::AuthenticatedAdmin {
        display_name,
        id: user_id,
        login,
        password_change_required,
        permissions,
        roles,
        session_id,
    })
}
