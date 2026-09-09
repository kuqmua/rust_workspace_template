pub async fn enrich_users_read_page(
    list_items: pg_crud_common::list_items::ListItems<crate::admin_users::AdminUsersRead>,
    list_items_primary_keys: pg_crud_common::list_items::ListItems<<pg_types_numeric::generate_pg_types_mod::I64AsNonNullBigSerialInitializationByPg as pg_crud_common::pg_type::PgType>::Read>,
    list_total: pg_crud_common::list_total::ListTotal,
    sqlx_pg_pool_ref: app_state::sqlx_pg_pool_ref::SqlxPgPoolRef<'_>,
) -> Result<
    crate::admin_users_read_page::AdminUsersReadPage,
    crate::admin_users_read_page_error::AdminUsersReadPageError,
> {
    let user_ids = Vec::from(list_items_primary_keys).into_iter().map(<pg_types_numeric::generate_pg_types_mod::I64AsNonNullBigSerialInitializationByPg as pg_crud_common::pg_type::PgType>::into_inner).collect::<Vec<_>>();
    let links = sqlx::query_as::<_, (i64, i64)>(constants_str::SERVER_ADMIN_LIST_USER_ROLE_IDS_SQL)
        .bind(user_ids.as_slice())
        .fetch_all(sqlx_pg_pool_ref.as_ref())
        .await
        .map_err(crate::sqlx_admin_error::SqlxAdminError::from)?;
    let mut role_ids_by_user = links.into_iter().try_fold(
        std::collections::BTreeMap::new(),
        |mut values, (user_id, role_id)| {
            values.entry(user_id).or_insert_with(Vec::new).push(
                server_admin_contract::admin_role_id::AdminRoleId::try_from(role_id)?,
            );
            Ok::<_, crate::admin_users_read_page_error::AdminUsersReadPageError>(values)
        },
    )?;
    let items = Vec::from(list_items)
        .into_iter()
        .zip(user_ids)
        .map(|(row, user_id)| {
            Ok(crate::admin_users_read_row::AdminUsersReadRow::new(
                row,
                server_admin_contract::admin_role_ids::AdminRoleIds::try_from(
                    role_ids_by_user.remove(&user_id).unwrap_or_default(),
                )?,
            ))
        })
        .collect::<Result<Vec<_>, crate::admin_users_read_page_error::AdminUsersReadPageError>>()?;
    let roles = crate::load_users_role_catalog::load_users_role_catalog(sqlx_pg_pool_ref).await?;
    Ok(crate::admin_users_read_page::AdminUsersReadPage::new(
        pg_crud_common::list_items::ListItems::from(items),
        roles,
        list_total,
    ))
}
