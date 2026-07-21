#![allow(clippy::single_call_fn)] // one bounded query serves the read-only table inspection boundary

fn generated_field_contracts(
    table: server_admin_contract::AdminDataTable,
) -> Option<frontend_contract::FieldContracts> {
    match table {
        server_admin_contract::AdminDataTable::Permissions => {
            Some(crate::generated_tables::AdminPermissions::frontend_fields())
        }
        server_admin_contract::AdminDataTable::RolePermissions => {
            Some(crate::generated_tables::AdminRolePermissions::frontend_fields())
        }
        server_admin_contract::AdminDataTable::Roles => {
            Some(crate::generated_tables::AdminRoles::frontend_fields())
        }
        server_admin_contract::AdminDataTable::SystemSettings => {
            Some(crate::generated_tables::AdminSystemSettings::frontend_fields())
        }
        server_admin_contract::AdminDataTable::UserRoles => {
            Some(crate::generated_tables::AdminUserRoles::frontend_fields())
        }
        server_admin_contract::AdminDataTable::Users => {
            Some(crate::generated_tables::AdminUsers::frontend_fields())
        }
        server_admin_contract::AdminDataTable::AccessSessions
        | server_admin_contract::AdminDataTable::AuditLog
        | server_admin_contract::AdminDataTable::CleanupStatus
        | server_admin_contract::AdminDataTable::LoginAttempts
        | server_admin_contract::AdminDataTable::RateLimits
        | server_admin_contract::AdminDataTable::RefreshTokens => None,
    }
}

fn data_columns(
    table: server_admin_contract::AdminDataTable,
    column_names: crate::StdAdminStrRef<'_>,
) -> Result<server_admin_contract::AdminDataColumns, super::AdminRepositoryError> {
    let generated_fields = generated_field_contracts(table);
    let columns = column_names
        .get()
        .split(',')
        .map(|raw_name| {
            let generated_field = generated_fields.as_ref().and_then(|fields| {
                AsRef::<[frontend_contract::FieldContract]>::as_ref(fields)
                    .iter()
                    .find(|field| field.name().as_ref() == raw_name)
            });
            let label_text = generated_field.map_or_else(
                || raw_name.to_owned(),
                |field| field.label().as_ref().to_owned(),
            );
            let input_kind =
                generated_field.map_or(server_admin_contract::AdminDataInputKind::Text, |field| {
                    server_admin_contract::AdminDataInputKind::from(
                        field.type_contract().input_kind(),
                    )
                });
            let label = server_admin_contract::AdminText::try_from(label_text)
                .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?;
            let name = server_admin_contract::AdminText::try_from(raw_name.to_owned())
                .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?;
            Ok(server_admin_contract::AdminDataColumn::new(
                input_kind, label, name,
            ))
        })
        .collect::<Result<Vec<_>, super::AdminRepositoryError>>()?;
    server_admin_contract::AdminDataColumns::try_from(columns)
        .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)
}

pub(crate) async fn read(
    pool: super::SqlxAdminRepositoryPoolRef<'_>,
    table: server_admin_contract::AdminDataTable,
    query: &server_admin_contract::AdminTableQuery,
) -> Result<server_admin_contract::AdminDataTableView, super::AdminRepositoryError> {
    let (column_names, count_sql, sql) = match table {
        server_admin_contract::AdminDataTable::AccessSessions => (
            str_constants::SERVER_ADMIN_DATA_REFRESH_TOKENS_COLUMNS,
            str_constants::SERVER_ADMIN_DATA_COUNT_ACCESS_SESSIONS_SQL,
            str_constants::SERVER_ADMIN_DATA_ACCESS_SESSIONS_SQL,
        ),
        server_admin_contract::AdminDataTable::AuditLog => (
            str_constants::SERVER_ADMIN_DATA_AUDIT_LOG_COLUMNS,
            str_constants::SERVER_ADMIN_DATA_COUNT_AUDIT_LOG_SQL,
            str_constants::SERVER_ADMIN_DATA_AUDIT_LOG_SQL,
        ),
        server_admin_contract::AdminDataTable::CleanupStatus => (
            str_constants::SERVER_ADMIN_DATA_CLEANUP_STATUS_COLUMNS,
            str_constants::SERVER_ADMIN_DATA_COUNT_CLEANUP_STATUS_SQL,
            str_constants::SERVER_ADMIN_DATA_CLEANUP_STATUS_SQL,
        ),
        server_admin_contract::AdminDataTable::LoginAttempts => (
            str_constants::SERVER_ADMIN_DATA_LOGIN_ATTEMPTS_COLUMNS,
            str_constants::SERVER_ADMIN_DATA_COUNT_LOGIN_ATTEMPTS_SQL,
            str_constants::SERVER_ADMIN_DATA_LOGIN_ATTEMPTS_SQL,
        ),
        server_admin_contract::AdminDataTable::Permissions => (
            str_constants::SERVER_ADMIN_DATA_PERMISSIONS_COLUMNS,
            str_constants::SERVER_ADMIN_DATA_COUNT_PERMISSIONS_SQL,
            str_constants::SERVER_ADMIN_DATA_PERMISSIONS_SQL,
        ),
        server_admin_contract::AdminDataTable::RateLimits => (
            str_constants::SERVER_ADMIN_DATA_RATE_LIMITS_COLUMNS,
            str_constants::SERVER_ADMIN_DATA_COUNT_RATE_LIMITS_SQL,
            str_constants::SERVER_ADMIN_DATA_RATE_LIMITS_SQL,
        ),
        server_admin_contract::AdminDataTable::RefreshTokens => (
            str_constants::SERVER_ADMIN_DATA_REFRESH_TOKENS_COLUMNS,
            str_constants::SERVER_ADMIN_DATA_COUNT_REFRESH_TOKENS_SQL,
            str_constants::SERVER_ADMIN_DATA_REFRESH_TOKENS_SQL,
        ),
        server_admin_contract::AdminDataTable::RolePermissions => (
            str_constants::SERVER_ADMIN_DATA_ROLE_PERMISSIONS_COLUMNS,
            str_constants::SERVER_ADMIN_DATA_COUNT_ROLE_PERMISSIONS_SQL,
            str_constants::SERVER_ADMIN_DATA_ROLE_PERMISSIONS_SQL,
        ),
        server_admin_contract::AdminDataTable::Roles => (
            str_constants::SERVER_ADMIN_DATA_ROLES_COLUMNS,
            str_constants::SERVER_ADMIN_DATA_COUNT_ROLES_SQL,
            str_constants::SERVER_ADMIN_DATA_ROLES_SQL,
        ),
        server_admin_contract::AdminDataTable::SystemSettings => (
            str_constants::SERVER_ADMIN_DATA_SYSTEM_SETTINGS_COLUMNS,
            str_constants::SERVER_ADMIN_DATA_COUNT_SYSTEM_SETTINGS_SQL,
            str_constants::SERVER_ADMIN_DATA_SYSTEM_SETTINGS_SQL,
        ),
        server_admin_contract::AdminDataTable::UserRoles => (
            str_constants::SERVER_ADMIN_DATA_USER_ROLES_COLUMNS,
            str_constants::SERVER_ADMIN_DATA_COUNT_USER_ROLES_SQL,
            str_constants::SERVER_ADMIN_DATA_USER_ROLES_SQL,
        ),
        server_admin_contract::AdminDataTable::Users => (
            str_constants::SERVER_ADMIN_DATA_USERS_COLUMNS,
            str_constants::SELECT_COUNT_ASTERISK_FROM_ADMIN_USERS,
            str_constants::SERVER_ADMIN_DATA_USERS_SQL,
        ),
    };
    let columns = data_columns(table, crate::StdAdminStrRef::from(column_names))?;
    let total = sqlx::query_scalar::<_, i64>(count_sql)
        .fetch_one(pool.0)
        .await
        .map_err(crate::SqlxAdminError::from)?;
    let rows = sqlx::query_scalar::<_, Vec<Option<String>>>(sql)
        .bind(i64::from(u16::from(query.limit())))
        .bind(i64::from(u32::from(query.offset())))
        .fetch_all(pool.0)
        .await
        .map_err(crate::SqlxAdminError::from)?;
    let items = rows
        .into_iter()
        .map(|row| {
            let values = row
                .into_iter()
                .map(|value| {
                    server_admin_contract::AdminText::try_from(
                        value.unwrap_or_else(|| str_constants::SERVER_ADMIN_DATA_NULL.to_owned()),
                    )
                    .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)
                })
                .collect::<Result<Vec<_>, super::AdminRepositoryError>>()?;
            server_admin_contract::AdminTexts::try_from(values)
                .map(server_admin_contract::AdminDataRow::new)
                .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)
        })
        .collect::<Result<Vec<_>, super::AdminRepositoryError>>()?;
    Ok(server_admin_contract::AdminDataTableView::new(
        columns,
        server_admin_contract::AdminDataRows::try_from(items)
            .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)?,
        table,
        super::page_total(super::AdminPageTotalCount::from(total))?,
    ))
}

#[cfg(test)]
mod tests {
    #[test]
    fn generated_table_fields_supply_client_column_metadata() {
        let columns = super::data_columns(
            server_admin_contract::AdminDataTable::Users,
            crate::StdAdminStrRef::from(str_constants::SERVER_ADMIN_DATA_USERS_COLUMNS),
        )
        .expect("f3c897af");
        let id = columns
            .as_slice()
            .iter()
            .find(|column| column.name().as_ref() == str_constants::SQL_NAMES_ID);
        assert!(id.is_some_and(|column| {
            column.input_kind() == server_admin_contract::AdminDataInputKind::Number
        }));
    }
}
