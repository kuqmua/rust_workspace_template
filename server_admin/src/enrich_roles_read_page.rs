#[allow(
    clippy::unused_async,
    reason = "the generated read-page enrichment contract requires an asynchronous function"
)]
pub async fn enrich_roles_read_page(
    list_items: pg_crud_common::list_items::ListItems<crate::admin_roles::AdminRolesRead>,
    _list_items_primary_keys: pg_crud_common::list_items::ListItems<<pg_types_numeric::generate_pg_types_mod::I64AsNonNullBigSerialInitializationByPg as pg_crud_common::pg_type::PgType>::Read>,
    list_total: pg_crud_common::list_total::ListTotal,
    _sqlx_pg_pool_ref: app_state::sqlx_pg_pool_ref::SqlxPgPoolRef<'_>,
) -> Result<
    crate::admin_roles_read_page::AdminRolesReadPage,
    crate::admin_roles_read_page_error::AdminRolesReadPageError,
> {
    Ok(crate::admin_roles_read_page::AdminRolesReadPage::new(
        list_items, list_total,
    ))
}
