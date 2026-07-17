#![allow(clippy::single_call_fn)] // each typed function owns one SQL bind/result contract

const RECORD_LOGIN_ATTEMPT: &str = "WITH attempt AS (INSERT INTO admin_login_attempts (login, ip_address, succeeded) VALUES ($1, $2, $3)) INSERT INTO admin_audit_log (user_login, action, resource, resource_id, request_id, succeeded, details) SELECT $1, 'sign_in', 'session', $1, $4, false, jsonb_build_object('ip_address', $2::INET::text) WHERE $3 = false";
const INSERT_AUDIT_SUCCESS: &str = "INSERT INTO admin_audit_log (user_id, user_login, action, resource, resource_id, request_id, succeeded, details) VALUES ($1, $2, $3, $4, $5, $6, true, $7)";
const QUERY_AUDIT_LOG: &str = "SELECT id, user_id, user_login, action, resource, resource_id, succeeded, details, created_at::text FROM admin_audit_log WHERE ($1::bigint IS NULL OR user_id = $1) AND ($2::text IS NULL OR action = $2) AND ($3::text IS NULL OR resource = $3) AND ($4::timestamptz IS NULL OR created_at >= $4::timestamptz) AND ($5::timestamptz IS NULL OR created_at <= $5::timestamptz) ORDER BY created_at DESC LIMIT 200";

pub(crate) async fn record_login_attempt(
    pool: super::SqlxAdminRepositoryPoolRef<'_>,
    login: &server_admin_contract::AdminLogin,
    peer: crate::auth::AdminPeerAddr,
    succeeded: crate::StdAdminBool,
    request_id: crate::UuidAdminValue,
) -> Result<(), crate::SqlxAdminError> {
    sqlx::query(RECORD_LOGIN_ATTEMPT)
        .bind(login.as_ref())
        .bind(peer.socket_addr().0.ip())
        .bind(succeeded.0)
        .bind(request_id.0)
        .execute(pool.0)
        .await
        .map_err(crate::SqlxAdminError::from)
        .map(drop)
}

pub(crate) async fn insert_audit_success(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    user_id: crate::AdminUserId,
    login: &server_admin_contract::AdminLogin,
    action: crate::AdminAuditAction,
    resource: crate::AdminAuditResource,
    resource_id: &crate::StdAdminString,
    request_id: crate::UuidAdminValue,
    details: &server_admin_contract::SerdeJsonAdminAuditDetails,
) -> Result<(), crate::SqlxAdminError> {
    sqlx::query(INSERT_AUDIT_SUCCESS)
        .bind(user_id.0)
        .bind(login.as_ref())
        .bind(action.as_str().as_ref())
        .bind(resource.as_str().as_ref())
        .bind(resource_id.as_ref())
        .bind(request_id.0)
        .bind(details.as_ref())
        .execute(connection.0)
        .await
        .map_err(crate::SqlxAdminError::from)
        .map(drop)
}

pub(crate) async fn query_audit_log(
    pool: super::SqlxAdminRepositoryPoolRef<'_>,
    query: crate::auth::AdminAuditQuery,
) -> Result<Vec<server_admin_contract::AdminAuditView>, super::AdminRepositoryError> {
    let (action, created_after, created_before, resource, user_id) = query.into_parts();
    let action_text = action.map(crate::AdminAuditAction::as_str);
    let resource_text = resource.map(crate::AdminAuditResource::as_str);
    sqlx::query_as::<
        _,
        (
            i64,
            Option<i64>,
            Option<String>,
            String,
            String,
            Option<String>,
            bool,
            Option<serde_json::Value>,
            String,
        ),
    >(QUERY_AUDIT_LOG)
    .bind(user_id.map(|value| value.0))
    .bind(action_text.map(|value| value.as_ref().to_owned()))
    .bind(resource_text.map(|value| value.as_ref().to_owned()))
    .bind(created_after.as_ref().map(|value| value.as_ref().as_str()))
    .bind(created_before.as_ref().map(|value| value.as_ref().as_str()))
    .fetch_all(pool.0)
    .await
    .map_err(crate::SqlxAdminError::from)?
    .into_iter()
    .map(|row| {
        Ok(server_admin_contract::AdminAuditView::new(
            server_admin_contract::AdminText::try_from(row.3)
                .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
            server_admin_contract::AdminAuditTimestamp::try_from(row.8)
                .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
            row.7
                .map(server_admin_contract::SerdeJsonAdminAuditDetails::try_from)
                .transpose()
                .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
            server_admin_contract::AdminAuditLogId::from(row.0),
            server_admin_contract::AdminText::try_from(row.4)
                .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
            row.5
                .map(server_admin_contract::AdminText::try_from)
                .transpose()
                .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
            server_admin_contract::AdminBool::from(row.6),
            row.1.map(server_admin_contract::AdminUserId::from),
            row.2
                .map(server_admin_contract::AdminLogin::try_from)
                .transpose()
                .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
        ))
    })
    .collect()
}
