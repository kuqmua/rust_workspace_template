pub(crate) fn admin_data_columns(
    admin_data_table: server_admin_contract::admin_data_table::AdminDataTable,
    admin_generated_table: Option<crate::admin_generated_table::AdminGeneratedTable>,
) -> Result<
    server_admin_contract::admin_data_columns::AdminDataColumns,
    crate::admin_repository_error::AdminRepositoryError,
> {
    let column_names = admin_data_table.spec().columns();
    let generated_fields = if admin_data_table
        == server_admin_contract::admin_data_table::AdminDataTable::AccessSessions
    {
        Some(crate::admin_access_sessions::AdminAccessSessions::frontend_fields())
    } else if admin_data_table == server_admin_contract::admin_data_table::AdminDataTable::AuditLog
    {
        Some(crate::admin_audit_log::AdminAuditLog::frontend_fields())
    } else if admin_data_table
        == server_admin_contract::admin_data_table::AdminDataTable::CleanupStatus
    {
        Some(crate::admin_cleanup_status::AdminCleanupStatus::frontend_fields())
    } else if admin_data_table
        == server_admin_contract::admin_data_table::AdminDataTable::LoginAttempts
    {
        Some(crate::admin_login_attempts::AdminLoginAttempts::frontend_fields())
    } else if admin_data_table
        == server_admin_contract::admin_data_table::AdminDataTable::RefreshTokens
    {
        Some(crate::admin_refresh_tokens::AdminRefreshTokens::frontend_fields())
    } else if admin_data_table
        == server_admin_contract::admin_data_table::AdminDataTable::RateLimits
    {
        Some(crate::admin_rate_limits::AdminRateLimits::frontend_fields())
    } else {
        admin_generated_table
            .map(crate::admin_generated_table::AdminGeneratedTable::field_contracts)
    };
    let columns = column_names
        .get()
        .split(',')
        .map(|raw_name| {
            let generated_field = generated_fields.as_ref().and_then(|fields| {
                AsRef::<[frontend_contract::field_contract::FieldContract]>::as_ref(fields)
                    .iter()
                    .find(|field| field.name().as_ref() == raw_name)
            });
            let label_text = generated_field.map_or_else(
                || raw_name.to_owned(),
                |field| field.label().as_ref().to_owned(),
            );
            let input_kind = generated_field
                .map_or(frontend_contract::input_kind::InputKind::Text, |field| {
                    field.type_contract().input_kind()
                });
            let raw_filters = generated_field.map_or_else(Vec::new, |field| {
                if field.readable()
                    == frontend_contract::field_capability::FieldCapability::Disabled
                {
                    Vec::new()
                } else {
                    field
                        .filters()
                        .iter()
                        .copied()
                        .map(server_admin_contract::admin_data_filter::AdminDataFilter::from)
                        .collect::<Vec<_>>()
                }
            });
            let filters =
                server_admin_contract::admin_data_filters::AdminDataFilters::try_from(raw_filters)
                    .map_err(|_error| {
                        crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue
                    })?;
            let label = server_admin_contract::admin_text::AdminText::try_from(label_text)
                .map_err(|_error| {
                    crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue
                })?;
            let name = server_admin_contract::admin_text::AdminText::try_from(raw_name.to_owned())
                .map_err(|_error| {
                    crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue
                })?;
            Ok(
                server_admin_contract::admin_data_column::AdminDataColumn::new(
                    filters, input_kind, label, name,
                ),
            )
        })
        .collect::<Result<Vec<_>, crate::admin_repository_error::AdminRepositoryError>>()?;
    server_admin_contract::admin_data_columns::AdminDataColumns::try_from(columns)
        .map_err(|_error| crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)
}
