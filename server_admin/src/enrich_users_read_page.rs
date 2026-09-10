pub async fn enrich_users_read_page(
    list_items: pg_crud_common::list_items::ListItems<crate::admin_users::AdminUsersRead>,
    list_items_primary_keys: pg_crud_common::list_items::ListItems<<pg_types_numeric::generate_pg_types_mod::I64AsNonNullBigSerialInitializationByPg as pg_crud_common::pg_type::PgType>::Read>,
    list_total: pg_crud_common::list_total::ListTotal,
    sqlx_pg_pool_ref: app_state::sqlx_pg_pool_ref::SqlxPgPoolRef<'_>,
) -> Result<
    crate::admin_users_read_page::AdminUsersReadPage,
    crate::admin_users_read_page_error::AdminUsersReadPageError,
> {
    let items = crate::enrich_assignment_rows::enrich_assignment_rows(
        list_items,
        list_items_primary_keys,
        app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(sqlx_pg_pool_ref.as_ref()),
        server_admin_core::std_admin_str_ref::StdAdminStrRef::from(
            constants_str::SERVER_ADMIN_LIST_USER_ROLE_IDS_SQL,
        ),
        |row, identifiers| {
            Ok::<_, crate::admin_users_read_page_error::AdminUsersReadPageError>(
                crate::admin_users_read_row::AdminUsersReadRow::new(
                    row,
                    server_admin_contract::admin_role_ids::AdminRoleIds::try_from(Vec::from(
                        identifiers,
                    ))?,
                ),
            )
        },
    )
    .await?;
    let roles = crate::load_users_role_catalog::load_users_role_catalog(sqlx_pg_pool_ref).await?;
    Ok(crate::admin_users_read_page::AdminUsersReadPage::new(
        items, roles, list_total,
    ))
}
