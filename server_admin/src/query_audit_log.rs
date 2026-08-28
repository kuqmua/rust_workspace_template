pub(crate) async fn query_audit_log(
    pool: crate::SqlxAdminRepositoryPoolRef<'_>,
    query: crate::domain_types::auth::AdminAuditQuery,
) -> Result<server_admin_contract::domain_types::AdminAuditPage, crate::AdminRepositoryError> {
    let parts = query.into_parts();
    let action_text = parts
        .get_action()
        .copied()
        .map(crate::domain_types::AdminAuditAction::as_str);
    let resource_text = parts
        .get_resource()
        .copied()
        .map(crate::domain_types::AdminAuditResource::as_str);
    let limit = usize::from(u16::from(*parts.get_limit()));
    let fetch_limit = i64::try_from(limit.saturating_add(constants_usize::ONE))
        .map_err(|_error| crate::AdminRepositoryError::InvalidStoredValue)?;
    let total =
        sqlx::query_scalar::<_, i64>(constants_str::SERVER_ADMIN_COUNT_FILTERED_AUDIT_LOG_SQL)
            .bind(
                parts
                    .get_user_id()
                    .copied()
                    .map(crate::domain_types::AdminUserId::get),
            )
            .bind(action_text.map(|value| value.as_ref().to_owned()))
            .bind(resource_text.map(|value| value.as_ref().to_owned()))
            .bind(
                parts
                    .get_created_after()
                    .map(|value| value.as_ref().as_str()),
            )
            .bind(
                parts
                    .get_created_before()
                    .map(|value| value.as_ref().as_str()),
            )
            .bind(parts.get_user_login().map(|value| value.as_ref().as_str()))
            .bind(parts.get_resource_id().map(|value| value.as_ref().as_str()))
            .bind(parts.get_succeeded().copied().map(bool::from))
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
    .bind(
        parts
            .get_user_id()
            .copied()
            .map(crate::domain_types::AdminUserId::get),
    )
    .bind(action_text.map(|value| value.as_ref().to_owned()))
    .bind(resource_text.map(|value| value.as_ref().to_owned()))
    .bind(
        parts
            .get_created_after()
            .map(|value| value.as_ref().as_str()),
    )
    .bind(
        parts
            .get_created_before()
            .map(|value| value.as_ref().as_str()),
    )
    .bind(
        parts
            .get_cursor_created_at()
            .map(|value| value.as_ref().as_str()),
    )
    .bind(parts.get_cursor_id().copied().map(i64::from))
    .bind(parts.get_user_login().map(|value| value.as_ref().as_str()))
    .bind(parts.get_resource_id().map(|value| value.as_ref().as_str()))
    .bind(parts.get_succeeded().copied().map(bool::from))
    .bind(fetch_limit)
    .bind(i64::from(u32::from(*parts.get_offset())))
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
                    .map_err(|_error| crate::AdminRepositoryError::InvalidStoredValue)?,
                server_admin_contract::domain_types::AdminAuditTimestamp::try_from(row.8)
                    .map_err(|_error| crate::AdminRepositoryError::InvalidStoredValue)?,
                row.7
                    .map(server_admin_contract::domain_types::SerdeJsonAdminAuditDetails::try_from)
                    .transpose()
                    .map_err(|_error| crate::AdminRepositoryError::InvalidStoredValue)?,
                server_admin_contract::domain_types::AdminAuditLogId::try_from(row.0)
                    .map_err(|_error| crate::AdminRepositoryError::InvalidStoredValue)?,
                server_admin_contract::domain_types::AdminText::try_from(row.4)
                    .map_err(|_error| crate::AdminRepositoryError::InvalidStoredValue)?,
                row.5
                    .map(server_admin_contract::domain_types::AdminText::try_from)
                    .transpose()
                    .map_err(|_error| crate::AdminRepositoryError::InvalidStoredValue)?,
                server_admin_contract::domain_types::AdminBool::from(row.6),
                row.1
                    .map(server_admin_contract::domain_types::AdminUserId::try_from)
                    .transpose()
                    .map_err(|_error| crate::AdminRepositoryError::InvalidStoredValue)?,
                row.2
                    .map(server_admin_contract::domain_types::AdminLogin::try_from)
                    .transpose()
                    .map_err(|_error| crate::AdminRepositoryError::InvalidStoredValue)?,
            ))
        })
        .collect::<Result<Vec<_>, crate::AdminRepositoryError>>()?;
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
            .map_err(|_error| crate::AdminRepositoryError::InvalidStoredValue)?,
        next_cursor,
        crate::repository_page_total(crate::AdminPageTotalCount::from(total))?,
    ))
}
