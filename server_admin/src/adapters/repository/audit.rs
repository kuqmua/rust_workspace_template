#![allow(clippy::single_call_fn)] // each typed function owns one SQL bind/result contract

pub(crate) async fn record_login_attempt(
    pool: super::SqlxAdminRepositoryPoolRef<'_>,
    login: &server_admin_contract::domain_types::AdminLogin,
    peer: crate::domain_types::auth::AdminPeerAddr,
    succeeded: crate::domain_types::StdAdminBool,
    request_id: crate::domain_types::UuidAdminValue,
) -> Result<(), crate::domain_types::SqlxAdminError> {
    sqlx::query(constants_str::SERVER_ADMIN_RECORD_LOGIN_ATTEMPT_SQL)
        .bind(login.as_ref())
        .bind(peer.socket_addr().get().ip())
        .bind(succeeded.get())
        .bind(request_id.get())
        .execute(pool.0)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .map(drop)
}

pub(crate) async fn insert_audit_success(
    connection: super::SqlxAdminRepositoryConnectionMutRef<'_>,
    user_id: crate::domain_types::AdminUserId,
    login: &server_admin_contract::domain_types::AdminLogin,
    action: crate::domain_types::AdminAuditAction,
    resource: crate::domain_types::AdminAuditResource,
    resource_id: &crate::domain_types::StdAdminString,
    request_id: crate::domain_types::UuidAdminValue,
    details: &server_admin_contract::domain_types::SerdeJsonAdminAuditDetails,
) -> Result<(), crate::domain_types::SqlxAdminError> {
    sqlx::query(constants_str::SERVER_ADMIN_INSERT_AUDIT_SUCCESS_SQL)
        .bind(user_id.get())
        .bind(login.as_ref())
        .bind(action.as_str().as_ref())
        .bind(resource.as_str().as_ref())
        .bind(resource_id.as_ref())
        .bind(request_id.get())
        .bind(details.as_ref())
        .execute(connection.0)
        .await
        .map_err(crate::domain_types::SqlxAdminError::from)
        .map(drop)
}
pub(crate) async fn query_audit_log(
    pool: super::SqlxAdminRepositoryPoolRef<'_>,
    query: crate::domain_types::auth::AdminAuditQuery,
) -> Result<server_admin_contract::domain_types::AdminAuditPage, super::AdminRepositoryError> {
    let parts = query.into_parts();
    let action_text = parts
        .action
        .map(crate::domain_types::AdminAuditAction::as_str);
    let resource_text = parts
        .resource
        .map(crate::domain_types::AdminAuditResource::as_str);
    let limit = usize::from(u16::from(parts.limit));
    let fetch_limit = i64::try_from(limit.saturating_add(constants_usize::ONE))
        .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?;
    let total =
        sqlx::query_scalar::<_, i64>(constants_str::SERVER_ADMIN_COUNT_FILTERED_AUDIT_LOG_SQL)
            .bind(parts.user_id.map(crate::domain_types::AdminUserId::get))
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
            .fetch_one(pool.0)
            .await
            .map_err(crate::domain_types::SqlxAdminError::from)?;
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
    >(constants_str::SERVER_ADMIN_PAGE_AUDIT_LOG_SQL)
    .bind(parts.user_id.map(crate::domain_types::AdminUserId::get))
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
    .bind(i64::from(u32::from(parts.offset)))
    .fetch_all(pool.0)
    .await
    .map_err(crate::domain_types::SqlxAdminError::from)?;
    let has_more = rows.len() > limit;
    let views = rows
        .into_iter()
        .take(limit)
        .map(|row| {
            Ok(server_admin_contract::domain_types::AdminAuditView::new(
                server_admin_contract::domain_types::AdminText::try_from(row.3)
                    .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
                server_admin_contract::domain_types::AdminAuditTimestamp::try_from(row.8)
                    .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
                row.7
                    .map(server_admin_contract::domain_types::SerdeJsonAdminAuditDetails::try_from)
                    .transpose()
                    .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
                server_admin_contract::domain_types::AdminAuditLogId::try_from(row.0)
                    .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
                server_admin_contract::domain_types::AdminText::try_from(row.4)
                    .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
                row.5
                    .map(server_admin_contract::domain_types::AdminText::try_from)
                    .transpose()
                    .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
                server_admin_contract::domain_types::AdminBool::from(row.6),
                row.1
                    .map(server_admin_contract::domain_types::AdminUserId::try_from)
                    .transpose()
                    .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
                row.2
                    .map(server_admin_contract::domain_types::AdminLogin::try_from)
                    .transpose()
                    .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
            ))
        })
        .collect::<Result<Vec<_>, super::AdminRepositoryError>>()?;
    let next_cursor = if has_more {
        views.last().map(|view| {
            server_admin_contract::domain_types::AdminAuditCursor::new(
                view.created_at().clone(),
                view.id(),
            )
        })
    } else {
        None
    };
    Ok(server_admin_contract::domain_types::AdminAuditPage::new(
        server_admin_contract::domain_types::AdminAuditViews::try_from(views)
            .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
        next_cursor,
        super::page_total(super::AdminPageTotalCount::from(total))?,
    ))
}
