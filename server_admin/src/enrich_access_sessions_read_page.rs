#[allow(
    clippy::unused_async,
    reason = "generated read page enrichment requires an asynchronous function signature"
)]
pub async fn enrich_access_sessions_read_page(
    list_items: pg_crud_common::list_items::ListItems<
        crate::admin_access_sessions::AdminAccessSessionsRead,
    >,
    _list_items_primary_keys: pg_crud_common::list_items::ListItems<
        <pg_types_text_misc::generate_pg_types_mod::SqlxTypesUuidUuidAsNonNullUuidV4InitializationByPg as pg_crud_common::pg_type::PgType>::Read,
    >,
    list_total: pg_crud_common::list_total::ListTotal,
    _sqlx_pg_pool_ref: app_state::sqlx_pg_pool_ref::SqlxPgPoolRef<'_>,
) -> Result<
    server_admin_contract::admin_data_table_view::AdminDataTableView,
    crate::admin_access_sessions_read_page_error::AdminAccessSessionsReadPageError,
> {
    crate::generated_data_table_view::generated_data_table_view(
        list_items,
        list_total,
        server_admin_contract::admin_data_table::AdminDataTable::AccessSessions,
    )
}
