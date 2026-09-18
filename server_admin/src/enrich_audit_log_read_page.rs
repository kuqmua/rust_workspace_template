#[allow(
    clippy::unused_async,
    reason = "generated read page enrichment requires an asynchronous function signature"
)]
pub async fn enrich_audit_log_read_page(
    list_items: pg_crud_common::list_items::ListItems<crate::admin_audit_log::AdminAuditLogRead>,
    _list_items_primary_keys: pg_crud_common::list_items::ListItems<
        <pg_types_numeric::generate_pg_types_mod::I64AsNonNullBigSerialInitializationByPg as pg_crud_common::pg_type::PgType>::Read,
    >,
    list_total: pg_crud_common::list_total::ListTotal,
    _sqlx_pg_pool_ref: app_state::sqlx_pg_pool_ref::SqlxPgPoolRef<'_>,
) -> Result<
    server_admin_contract::admin_data_table_view::AdminDataTableView,
    crate::admin_audit_log_read_page_error::AdminAuditLogReadPageError,
> {
    let admin_data_table = server_admin_contract::admin_data_table::AdminDataTable::AuditLog;
    let admin_generated_table =
        crate::admin_generated_table::AdminGeneratedTable::for_data_table(admin_data_table);
    let all_columns =
        crate::admin_data_columns::admin_data_columns(admin_data_table, admin_generated_table)?;
    let fields = crate::admin_audit_log::AdminAuditLog::frontend_fields();
    let columns = server_admin_contract::admin_data_columns::AdminDataColumns::try_from(
        all_columns
            .as_slice()
            .iter()
            .filter(|column| {
                fields.as_ref().iter().any(|field| {
                    field.name().as_ref() == column.name().as_ref()
                        && field.readable()
                            == frontend_contract::field_capability::FieldCapability::Enabled
                })
            })
            .cloned()
            .collect::<Vec<_>>(),
    )?;
    let items = Vec::from(list_items)
        .into_iter()
        .map(|item| {
            let serialized = serde_json::to_value(item)
                .map_err(server_runtime_http::serde_json_error::SerdeJsonError::from)?;
            let object = serialized.as_object().ok_or(
                crate::admin_audit_log_read_page_error::AdminAuditLogReadPageError::StoredValue,
            )?;
            let values = columns
                .as_slice()
                .iter()
                .map(|column| {
                    let value = object
                        .get(column.name().as_ref())
                        .and_then(serde_json::Value::as_object)
                        .and_then(|explicit| explicit.get(constants_str::PG_CRUD_VALUES_FIELD))
                        .ok_or(
                            crate::admin_audit_log_read_page_error::AdminAuditLogReadPageError::StoredValue,
                        )?;
                    let text = match value {
                        serde_json::Value::Null => constants_str::SERVER_ADMIN_DATA_NULL.to_owned(),
                        serde_json::Value::String(value) => value.clone(),
                        serialized_value @ (serde_json::Value::Bool(_)
                        | serde_json::Value::Number(_)
                        | serde_json::Value::Array(_)
                        | serde_json::Value::Object(_)) => serialized_value.to_string(),
                    };
                    server_admin_contract::admin_text::AdminText::try_from(text).map_err(
                        crate::admin_audit_log_read_page_error::AdminAuditLogReadPageError::from,
                    )
                })
                .collect::<Result<Vec<_>, crate::admin_audit_log_read_page_error::AdminAuditLogReadPageError>>()?;
            server_admin_contract::admin_texts::AdminTexts::try_from(values)
                .map(server_admin_contract::admin_data_row::AdminDataRow::new)
                .map_err(crate::admin_audit_log_read_page_error::AdminAuditLogReadPageError::from)
        })
        .collect::<Result<Vec<_>, crate::admin_audit_log_read_page_error::AdminAuditLogReadPageError>>()?;
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
