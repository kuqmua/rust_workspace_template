#![allow(clippy::single_call_fn)] // each typed function owns one SQL bind/result contract

pub(crate) async fn record_login_attempt(
    pool: super::SqlxAdminRepositoryPoolRef<'_>,
    login: &server_admin_contract::AdminLogin,
    peer: crate::auth::AdminPeerAddr,
    succeeded: crate::StdAdminBool,
    request_id: crate::UuidAdminValue,
) -> Result<(), crate::SqlxAdminError> {
    sqlx::query(str_constants::SERVER_ADMIN_RECORD_LOGIN_ATTEMPT_SQL)
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
    sqlx::query(str_constants::SERVER_ADMIN_INSERT_AUDIT_SUCCESS_SQL)
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
) -> Result<server_admin_contract::AdminAuditPage, super::AdminRepositoryError> {
    let parts = query.into_parts();
    let action_text = parts.action.map(crate::AdminAuditAction::as_str);
    let resource_text = parts.resource.map(crate::AdminAuditResource::as_str);
    let limit = usize::from(u16::from(parts.limit));
    let fetch_limit = i64::try_from(limit.saturating_add(1usize))
        .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?;
    let rows = sqlx::query_as::<
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
    >(str_constants::SERVER_ADMIN_PAGE_AUDIT_LOG_SQL)
    .bind(parts.user_id.map(|value| value.0))
    .bind(action_text.map(|value| value.as_ref().to_owned()))
    .bind(resource_text.map(|value| value.as_ref().to_owned()))
    .bind(
        parts
            .created_after
            .as_ref()
            .map(|value| value.as_ref().as_str()),
    )
    .bind(
        parts
            .created_before
            .as_ref()
            .map(|value| value.as_ref().as_str()),
    )
    .bind(
        parts
            .cursor_created_at
            .as_ref()
            .map(|value| value.as_ref().as_str()),
    )
    .bind(parts.cursor_id.map(i64::from))
    .bind(
        parts
            .user_login
            .as_ref()
            .map(|value| value.as_ref().as_str()),
    )
    .bind(
        parts
            .resource_id
            .as_ref()
            .map(|value| value.as_ref().as_str()),
    )
    .bind(parts.succeeded.map(bool::from))
    .bind(fetch_limit)
    .fetch_all(pool.0)
    .await
    .map_err(crate::SqlxAdminError::from)?;
    let has_more = rows.len() > limit;
    let views = rows
        .into_iter()
        .take(limit)
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
        .collect::<Result<Vec<_>, super::AdminRepositoryError>>()?;
    let next_cursor = if has_more {
        views.last().map(|view| {
            server_admin_contract::AdminAuditCursor::new(view.created_at().clone(), view.id())
        })
    } else {
        None
    };
    Ok(server_admin_contract::AdminAuditPage::new(
        views,
        next_cursor,
    ))
}
