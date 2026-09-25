pub(crate) fn generated_data_table_view<Item, Error>(
    list_items: pg_crud_common::list_items::ListItems<Item>,
    list_total: pg_crud_common::list_total::ListTotal,
    admin_data_table: server_admin_contract::admin_data_table::AdminDataTable,
    columns: Option<server_admin_contract::admin_data_columns::AdminDataColumns>,
) -> Result<server_admin_contract::admin_data_table_view::AdminDataTableView, Error>
where
    Item: serde::Serialize,
    Error: From<crate::admin_repository_error::AdminRepositoryError>
        + From<server_admin_contract::admin_collection_error::AdminCollectionError>
        + From<server_admin_contract::admin_text::AdminTextTryFromStringError>
        + From<server_runtime_http::serde_json_error::SerdeJsonError>
        + From<pg_crud_common::list_total_error::ListTotalError>,
{
    let admin_generated_table =
        crate::admin_generated_table::AdminGeneratedTable::for_data_table(admin_data_table);
    let derived_columns = columns.is_none();
    let columns = match columns {
        Some(columns) => columns,
        None => {
            crate::admin_data_columns::admin_data_columns(admin_data_table, admin_generated_table)?
        }
    };
    let list_items = Vec::from(list_items);
    let columns = if derived_columns {
        match list_items.first() {
            Some(item) => {
                let serialized = serde_json::to_value(item)
                    .map_err(server_runtime_http::serde_json_error::SerdeJsonError::from)?;
                let object = serialized.as_object().ok_or(
                    crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue,
                )?;
                server_admin_contract::admin_data_columns::AdminDataColumns::try_from(
                    columns
                        .as_slice()
                        .iter()
                        .filter(|column| object.contains_key(column.name().as_ref()))
                        .cloned()
                        .collect::<Vec<_>>(),
                )?
            }
            None => columns,
        }
    } else {
        columns
    };
    let items = list_items
        .into_iter()
        .map(|item| {
            let serialized = serde_json::to_value(item)
                .map_err(server_runtime_http::serde_json_error::SerdeJsonError::from)?;
            let object = serialized
                .as_object()
                .ok_or(crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)?;
            let values = columns
                .as_slice()
                .iter()
                .map(|column| {
                    let value = object
                        .get(column.name().as_ref())
                        .and_then(serde_json::Value::as_object)
                        .and_then(|explicit| explicit.get(constants_str::VALUE_CD42404D))
                        .ok_or(
                            crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue,
                        )?;
                    let text = match value {
                        serde_json::Value::Null => constants_str::SERVER_ADMIN_DATA_NULL.to_owned(),
                        serde_json::Value::String(value) => value.clone(),
                        serialized_value @ (serde_json::Value::Bool(_)
                        | serde_json::Value::Number(_)
                        | serde_json::Value::Array(_)
                        | serde_json::Value::Object(_)) => serialized_value.to_string(),
                    };
                    server_admin_contract::admin_text::AdminText::try_from(text)
                        .map_err(Error::from)
                })
                .collect::<Result<Vec<_>, Error>>()?;
            server_admin_contract::admin_texts::AdminTexts::try_from(values)
                .map(server_admin_contract::admin_data_row::AdminDataRow::new)
                .map_err(Error::from)
        })
        .collect::<Result<Vec<_>, Error>>()?;
    let total = crate::repository_page_total::repository_page_total(
        crate::admin_page_total_count::AdminPageTotalCount::from(i64::from(list_total)),
    )?;
    Ok(
        server_admin_contract::admin_data_table_view::AdminDataTableView::new(
            columns,
            server_admin_contract::admin_data_rows::AdminDataRows::try_from(items)?,
            admin_data_table,
            total,
        ),
    )
}
