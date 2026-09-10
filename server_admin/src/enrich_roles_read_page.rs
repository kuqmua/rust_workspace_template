pub async fn enrich_roles_read_page(
    list_items: pg_crud_common::list_items::ListItems<crate::admin_roles::AdminRolesRead>,
    list_items_primary_keys: pg_crud_common::list_items::ListItems<<pg_types_numeric::generate_pg_types_mod::I64AsNonNullBigSerialInitializationByPg as pg_crud_common::pg_type::PgType>::Read>,
    list_total: pg_crud_common::list_total::ListTotal,
    sqlx_pg_pool_ref: app_state::sqlx_pg_pool_ref::SqlxPgPoolRef<'_>,
) -> Result<
    crate::admin_roles_read_page::AdminRolesReadPage,
    crate::admin_roles_read_page_error::AdminRolesReadPageError,
> {
    let items = crate::enrich_assignment_rows::enrich_assignment_rows(
        list_items,
        list_items_primary_keys,
        app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(sqlx_pg_pool_ref.as_ref()),
        server_admin_core::std_admin_str_ref::StdAdminStrRef::from(
            constants_str::SERVER_ADMIN_LIST_ROLE_PERMISSION_IDS_SQL,
        ),
        |row, identifiers| {
            Ok::<_, crate::admin_roles_read_page_error::AdminRolesReadPageError>(
                crate::admin_roles_read_row::AdminRolesReadRow::new(
                    row,
                    server_admin_contract::admin_permission_ids::AdminPermissionIds::try_from(
                        Vec::from(identifiers),
                    )?,
                ),
            )
        },
    )
    .await?;
    let permissions =
        crate::load_role_permission_catalog::load_role_permission_catalog(sqlx_pg_pool_ref).await?;
    Ok(crate::admin_roles_read_page::AdminRolesReadPage::new(
        items,
        permissions,
        list_total,
    ))
}
