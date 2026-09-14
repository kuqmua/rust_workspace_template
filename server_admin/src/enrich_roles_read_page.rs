pub async fn enrich_roles_read_page(
    list_items: pg_crud_common::list_items::ListItems<crate::admin_roles::AdminRolesRead>,
    list_items_primary_keys: pg_crud_common::list_items::ListItems<<pg_types_numeric::generate_pg_types_mod::I64AsNonNullBigSerialInitializationByPg as pg_crud_common::pg_type::PgType>::Read>,
    list_total: pg_crud_common::list_total::ListTotal,
    sqlx_pg_pool_ref: app_state::sqlx_pg_pool_ref::SqlxPgPoolRef<'_>,
    admin_table_query: Option<&server_admin_contract::admin_table_query::AdminTableQuery>,
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
    let (permissions, permissions_total) = if let Some(admin_table_query) = admin_table_query {
        if !admin_table_query.sort().as_ref().is_empty() {
            server_admin_contract::admin_table_sort_field::AdminTableSortField::try_from_key(
                &server_admin_contract::admin_table_sort_field::AdminTableSortField::PERMISSION,
                server_admin_contract::admin_table_sort_key_ref::AdminTableSortKeyRef::from(
                    admin_table_query.sort().as_ref(),
                ),
            )
            .map(drop)?;
        }
        let search = admin_table_query.search().as_ref();
        let permissions_total = sqlx::query_scalar::<_, i64>(
            constants_str::SERVER_ADMIN_COUNT_FILTERED_PERMISSIONS_SQL,
        )
        .bind(search)
        .fetch_one(sqlx_pg_pool_ref.as_ref())
        .await
        .map_err(crate::sqlx_admin_error::SqlxAdminError::from)?;
        let permissions_total = pg_crud_common::list_total::ListTotal::try_from(permissions_total)?;
        let permissions = sqlx::query_as::<_, (i64, String)>(
            constants_str::SERVER_ADMIN_PAGE_PERMISSIONS_SQL,
        )
        .bind(search)
        .bind(admin_table_query.sort().as_ref())
        .bind(admin_table_query.direction().as_ref())
        .bind(i64::from(u16::from(admin_table_query.limit())))
        .bind(i64::from(u32::from(admin_table_query.offset())))
        .fetch_all(sqlx_pg_pool_ref.as_ref())
        .await
        .map_err(crate::sqlx_admin_error::SqlxAdminError::from)?
        .into_iter()
        .map(|(id, name)| {
            Ok(server_admin_contract::admin_permission_summary::AdminPermissionSummary::new(
                server_admin_contract::admin_permission_id::AdminPermissionId::try_from(id)?,
                server_admin_contract::admin_permission_value::AdminPermissionValue::try_from(name)
                    .map_err(|_error| crate::admin_roles_read_page_error::AdminRolesReadPageError::StoredPermission)?,
            ))
        })
        .collect::<Result<Vec<_>, crate::admin_roles_read_page_error::AdminRolesReadPageError>>()?;
        (
            server_admin_contract::admin_permission_summaries::AdminPermissionSummaries::try_from(
                permissions,
            )?,
            permissions_total,
        )
    } else {
        let permissions =
            crate::load_role_permission_catalog::load_role_permission_catalog(sqlx_pg_pool_ref)
                .await?;
        let permissions_total = sqlx::query_scalar::<_, i64>(
            constants_str::SERVER_ADMIN_COUNT_FILTERED_PERMISSIONS_SQL,
        )
        .bind(constants_str::EMPTY)
        .fetch_one(sqlx_pg_pool_ref.as_ref())
        .await
        .map_err(crate::sqlx_admin_error::SqlxAdminError::from)?;
        let permissions_total = pg_crud_common::list_total::ListTotal::try_from(permissions_total)?;
        (permissions, permissions_total)
    };
    let (items, list_total) = if admin_table_query.is_some() {
        (
            pg_crud_common::list_items::ListItems::from(Vec::new()),
            pg_crud_common::list_total::ListTotal::from(0u32),
        )
    } else {
        (items, list_total)
    };
    Ok(crate::admin_roles_read_page::AdminRolesReadPage::new(
        items,
        permissions,
        list_total,
        permissions_total,
    ))
}
