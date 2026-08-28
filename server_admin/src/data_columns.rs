pub(crate) fn data_columns(
    table: server_admin_contract::domain_types::AdminDataTable,
    column_names: server_admin_contract::domain_types::AdminDataColumnsCsvRef<'_>,
) -> Result<server_admin_contract::domain_types::AdminDataColumns, crate::AdminRepositoryError> {
    let generated_fields =
        crate::domain_types::generated_tables::AdminGeneratedTable::for_data_table(table)
            .map(crate::domain_types::generated_tables::AdminGeneratedTable::field_contracts);
    let columns = column_names
        .get()
        .split(',')
        .map(|raw_name| {
            let generated_field = generated_fields.as_ref().and_then(|fields| {
                AsRef::<[frontend_contract::domain_types::FieldContract]>::as_ref(fields)
                    .iter()
                    .find(|field| field.name().as_ref() == raw_name)
            });
            let label_text = generated_field.map_or_else(
                || raw_name.to_owned(),
                |field| field.label().as_ref().to_owned(),
            );
            let input_kind = generated_field
                .map_or(frontend_contract::domain_types::InputKind::Text, |field| {
                    field.type_contract().input_kind()
                });
            let raw_filters = generated_field.map_or_else(Vec::new, |field| {
                field
                    .filters()
                    .iter()
                    .copied()
                    .map(server_admin_contract::domain_types::AdminDataFilter::from)
                    .collect::<Vec<_>>()
            });
            let filters =
                server_admin_contract::domain_types::AdminDataFilters::try_from(raw_filters)
                    .map_err(|_error| crate::AdminRepositoryError::InvalidStoredValue)?;
            let label = server_admin_contract::domain_types::AdminText::try_from(label_text)
                .map_err(|_error| crate::AdminRepositoryError::InvalidStoredValue)?;
            let name =
                server_admin_contract::domain_types::AdminText::try_from(raw_name.to_owned())
                    .map_err(|_error| crate::AdminRepositoryError::InvalidStoredValue)?;
            Ok(server_admin_contract::domain_types::AdminDataColumn::new(
                filters, input_kind, label, name,
            ))
        })
        .collect::<Result<Vec<_>, crate::AdminRepositoryError>>()?;
    server_admin_contract::domain_types::AdminDataColumns::try_from(columns)
        .map_err(|_error| crate::AdminRepositoryError::InvalidStoredValue)
}
