pub(crate) fn admin_sessions_table_view(
    admin_sessions_page: &server_admin_contract::admin_sessions_page::AdminSessionsPage,
) -> Result<
    server_admin_contract::admin_data_table_view::AdminDataTableView,
    crate::admin_table_load_error::AdminTableLoadError,
> {
    let columns = [
        (
            constants_str::SQL_NAMES_ID,
            frontend_contract::input_kind::InputKind::Uuid,
            vec![
                frontend_contract::filter_operation::FilterOperation::Eq,
                frontend_contract::filter_operation::FilterOperation::In,
            ],
        ),
        (
            constants_str::CREATED_AT,
            frontend_contract::input_kind::InputKind::DateTime,
            vec![
                frontend_contract::filter_operation::FilterOperation::Eq,
                frontend_contract::filter_operation::FilterOperation::Before,
                frontend_contract::filter_operation::FilterOperation::Between,
            ],
        ),
        (
            constants_str::EXPIRES_AT,
            frontend_contract::input_kind::InputKind::DateTime,
            vec![
                frontend_contract::filter_operation::FilterOperation::Eq,
                frontend_contract::filter_operation::FilterOperation::Before,
                frontend_contract::filter_operation::FilterOperation::Between,
            ],
        ),
        (
            constants_str::CURRENT,
            frontend_contract::input_kind::InputKind::Checkbox,
            vec![frontend_contract::filter_operation::FilterOperation::Eq],
        ),
    ]
    .into_iter()
    .map(|(name, input_kind, filters)| {
        let name = server_admin_contract::admin_text::AdminText::try_from(name.to_owned())?;
        let filters = server_admin_contract::admin_data_filters::AdminDataFilters::try_from(
            filters
                .into_iter()
                .map(server_admin_contract::admin_data_filter::AdminDataFilter::from)
                .collect::<Vec<_>>(),
        )?;
        Ok(
            server_admin_contract::admin_data_column::AdminDataColumn::new(
                filters,
                input_kind,
                name.clone(),
                name,
            ),
        )
    })
    .collect::<Result<Vec<_>, crate::admin_table_load_error::AdminTableLoadError>>()?;
    let items = admin_sessions_page
        .items()
        .iter()
        .map(|item| {
            let values = [
                item.id().to_string(),
                item.created_at().to_string(),
                item.expires_at().to_string(),
                item.is_current().to_string(),
            ]
            .into_iter()
            .map(server_admin_contract::admin_text::AdminText::try_from)
            .collect::<Result<Vec<_>, _>>()?;
            Ok(server_admin_contract::admin_data_row::AdminDataRow::new(
                server_admin_contract::admin_texts::AdminTexts::try_from(values)?,
            ))
        })
        .collect::<Result<Vec<_>, crate::admin_table_load_error::AdminTableLoadError>>()?;
    Ok(
        server_admin_contract::admin_data_table_view::AdminDataTableView::new(
            server_admin_contract::admin_data_columns::AdminDataColumns::try_from(columns)?,
            server_admin_contract::admin_data_rows::AdminDataRows::try_from(items)?,
            server_admin_contract::admin_data_table::AdminDataTable::AccessSessions,
            admin_sessions_page.total(),
        ),
    )
}
