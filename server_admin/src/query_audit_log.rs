pub(crate) async fn query_audit_log(
    pool: crate::sqlx_admin_repository_pool_ref::SqlxAdminRepositoryPoolRef<'_>,
    query: crate::admin_audit_query::AdminAuditQuery,
) -> Result<
    server_admin_contract::admin_audit_page::AdminAuditPage,
    crate::admin_repository_error::AdminRepositoryError,
> {
    let parts = query.into_parts();
    let action_text = parts
        .get_action()
        .copied()
        .map(crate::admin_audit_action::AdminAuditAction::as_str);
    let resource_text = parts
        .get_resource()
        .copied()
        .map(crate::admin_audit_resource::AdminAuditResource::as_str);
    let limit = usize::from(u16::from(*parts.get_limit()));
    let fetch_limit =
        i64::try_from(limit.saturating_add(constants_usize::ONE)).map_err(|_error| {
            crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue
        })?;
    let total = sqlx::query_scalar::<_, i64>(
        constants_str::integration_fixtures::SERVER_ADMIN_COUNT_FILTERED_AUDIT_LOG_SQL,
    )
    .bind(
        parts
            .get_user_id()
            .copied()
            .map(server_admin_core::admin_user_id::AdminUserId::get),
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
    .map_err(crate::sqlx_admin_error::SqlxAdminError::from)?;
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
    >(constants_str::integration_fixtures::SERVER_ADMIN_PAGE_AUDIT_LOG_SQL)
    .bind(
        parts
            .get_user_id()
            .copied()
            .map(server_admin_core::admin_user_id::AdminUserId::get),
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
    .map_err(crate::sqlx_admin_error::SqlxAdminError::from)?;
    let has_more = rows.len() > limit;
    let views = rows
        .into_iter()
        .take(limit)
        .map(|row| {
            Ok(server_admin_contract::admin_audit_view::AdminAuditView::new(
                server_admin_contract::admin_text::AdminText::try_from(row.3).map_err(
                    |_error| {
                        crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue
                    },
                )?,
                server_admin_contract::admin_audit_timestamp::AdminAuditTimestamp::try_from(row.8).map_err(
                    |_error| {
                        crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue
                    },
                )?,
                row.7
                    .map(server_admin_contract::serde_json_admin_audit_details::SerdeJsonAdminAuditDetails::try_from)
                    .transpose()
                    .map_err(|_error| {
                        crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue
                    })?,
                server_admin_contract::admin_audit_log_id::AdminAuditLogId::try_from(row.0).map_err(
                    |_error| {
                        crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue
                    },
                )?,
                server_admin_contract::admin_text::AdminText::try_from(row.4).map_err(
                    |_error| {
                        crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue
                    },
                )?,
                row.5
                    .map(server_admin_contract::admin_text::AdminText::try_from)
                    .transpose()
                    .map_err(|_error| {
                        crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue
                    })?,
                server_admin_contract::admin_bool::AdminBool::from(row.6),
                row.1
                    .map(server_admin_contract::admin_user_id::AdminUserId::try_from)
                    .transpose()
                    .map_err(|_error| {
                        crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue
                    })?,
                row.2
                    .map(server_admin_contract::admin_login::AdminLogin::try_from)
                    .transpose()
                    .map_err(|_error| {
                        crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue
                    })?,
            ))
        })
        .collect::<Result<Vec<_>, crate::admin_repository_error::AdminRepositoryError>>()?;
    let next_cursor = if has_more {
        views.last().map(|view| {
            server_admin_contract::admin_audit_cursor::AdminAuditCursor::new(
                view.created_at().clone(),
                view.id(),
            )
        })
    } else {
        None
    };
    Ok(
        server_admin_contract::admin_audit_page::AdminAuditPage::new(
            server_admin_contract::admin_audit_views::AdminAuditViews::try_from(views).map_err(
                |_error| crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue,
            )?,
            next_cursor,
            crate::repository_page_total::repository_page_total(
                crate::admin_page_total_count::AdminPageTotalCount::from(total),
            )?,
        ),
    )
}
