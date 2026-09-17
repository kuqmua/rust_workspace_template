#[allow(
    clippy::unused_async,
    reason = "the generated read-page enrichment contract requires an asynchronous function"
)]
pub async fn enrich_permissions_read_page(
    list_items: pg_crud_common::list_items::ListItems<
        crate::admin_permissions::AdminPermissionsRead,
    >,
    list_items_primary_keys: pg_crud_common::list_items::ListItems<<pg_types_numeric::generate_pg_types_mod::I64AsNonNullBigSerialInitializationByPg as pg_crud_common::pg_type::PgType>::Read>,
    list_total: pg_crud_common::list_total::ListTotal,
    _sqlx_pg_pool_ref: app_state::sqlx_pg_pool_ref::SqlxPgPoolRef<'_>,
) -> Result<
    server_admin_contract::admin_permissions_page::AdminPermissionsPage,
    crate::admin_permissions_read_page_error::AdminPermissionsReadPageError,
> {
    let items = Vec::from(list_items)
        .into_iter()
        .zip(Vec::from(list_items_primary_keys))
        .map(|(item, id)| {
            let optional_name = item.get_name();
            let name = optional_name.as_ref().ok_or(
                crate::admin_permissions_read_page_error::AdminPermissionsReadPageError::MissingName,
            )?;
            Ok(server_admin_contract::admin_permission_summary::AdminPermissionSummary::new(
                server_admin_contract::admin_permission_id::AdminPermissionId::try_from(
                    <pg_types_numeric::generate_pg_types_mod::I64AsNonNullBigSerialInitializationByPg as pg_crud_common::pg_type::PgType>::into_inner(id),
                )?,
                match server_admin_contract::admin_permission_value::AdminPermissionValue::try_from(
                    <pg_types_text_misc::generate_pg_types_mod::StringAsNonNullText as pg_crud_common::pg_type::PgType>::into_inner(name.get_value().clone()),
                ) {
                    Ok(admin_permission_value) => admin_permission_value,
                    Err(_) => return Err(crate::admin_permissions_read_page_error::AdminPermissionsReadPageError::StoredPermission),
                },
            ))
        })
        .collect::<Result<Vec<_>, crate::admin_permissions_read_page_error::AdminPermissionsReadPageError>>()?;
    let Ok(total) = u64::try_from(i64::from(list_total)) else {
        return Err(
            crate::admin_permissions_read_page_error::AdminPermissionsReadPageError::TotalConversion,
        );
    };
    Ok(
        server_admin_contract::admin_permissions_page::AdminPermissionsPage::new(
            server_admin_contract::admin_permission_summaries::AdminPermissionSummaries::try_from(
                items,
            )?,
            server_admin_contract::admin_page_total::AdminPageTotal::from(total),
        ),
    )
}
