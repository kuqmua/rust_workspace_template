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
    crate::generated_data_table_view::generated_data_table_view(
        list_items,
        list_total,
        admin_data_table,
        Some(columns),
    )
}
